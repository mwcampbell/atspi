mod serde_signature;
mod xml_types;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, DeriveInput, ItemStruct, Lit, Meta, MetaNameValue,
    NestedMeta, Type,
};

enum FromZbusMessageParam {
    Invalid,
    Body(Type),
		Member(String),
}

impl FromZbusMessageParam {
    fn from(key: String, val: String) -> Self {
        match (key.as_str(), val.as_str()) {
            ("body", tp) => Self::Body(
                syn::parse_str(tp)
                    .expect("The value given to the 'body' parameter must be a valid type."),
            ),
						("member", mem) => Self::Member(mem.to_string()),
            _ => Self::Invalid,
        }
    }
}

//
// Derive macro for that implements TryFrom<Event> on a per name / member basis.
//

#[proc_macro_derive(TrySignify)]
pub fn try_from_atspi_event_to_signified_type(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    // Extract the name of the struct
    let name = &ident;

    // Remove the suffix "Event" from the name of the struct
    let name_string = name.to_string();
    let member = name_string.strip_suffix("Event").unwrap();

    // Generate the expanded code
    let expanded = quote! {
        impl TryFrom<AtspiEvent> for #name {
            type Error = crate::AtspiError;

            fn try_from(event: AtspiEvent) -> Result<Self, Self::Error> {
                let event_member: MemberName = event
                    .member()
                    .ok_or(AtspiError::MemberMatch("event w/o member".to_string()))?;

                let member = MemberName::from_static_str(#member)?;

                if event_member != member {
                    let error = format!("message member: {:?} != member: {:?}", event_member, member);
                    return Err(AtspiError::MemberMatch(error));
                };

                Ok( Self( event ) )
           }
        }

        impl<'a> Signified for #name {
            type Inner = AtspiEvent;
            fn inner(&self) -> &Self::Inner {
                &self.0
            }

            fn properties(&self) -> &HashMap<String, OwnedValue> {
                self.0.properties()
            }
        }
						impl #name {
							pub fn kind(&self) -> &str {
								self.inner().kind()
							}
						}
            impl GenericEvent for #name {
                    /// Bus message.
                    #[must_use]
                    fn message(&self) -> &Arc<Message> {
                            &self.inner().message()
                    }

                    /// For now this returns the full interface name because the lifetimes in [`zbus_names`][zbus::names] are
                    /// wrong such that the `&str` you can get from a
                    /// [`zbus_names::InterfaceName`][zbus::names::InterfaceName] is tied to the lifetime of that
                    /// name, not to the lifetime of the message as it should be. In future, this will return only
                    /// the last component of the interface name (I.E. "Object" from
                    /// "org.a11y.atspi.Event.Object").
                    #[must_use]
                    fn interface(&self) -> Option<zbus_names::InterfaceName<'_>> {
                            self.inner().interface()
                    }

                    /// Identifies this event's interface member name.
                    #[must_use]
                    fn member(&self) -> Option<MemberName<'_>> {
                            self.inner().member()
                    }

                    /// The object path to the object where the signal is emitted from.
                    #[must_use]
                    fn path(&self) -> std::option::Option<zbus::zvariant::OwnedObjectPath> {
                            self.inner().path()
                    }

                    /// Identifies the `sender` of the event.
                    /// # Errors
                    /// - when deserializeing the header failed, or
                    /// - When `zbus::get_field!` finds that 'sender' is an invalid field.
                    fn sender(&self) -> Result<Option<zbus::names::UniqueName>, crate::AtspiError> {
                            self.inner().sender()
                    }
                }
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}

//#[proc_macro_derive(TryFromMessage)]
#[proc_macro_attribute]
pub fn try_from_zbus_message(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    // Parse the input token stream into a syntax tree
    let name = item_struct.ident.clone();

    // Remove the suffix "Event" from the name of the struct
    let name_string = name.to_string();

    let args_parsed: Vec<FromZbusMessageParam> = parse_macro_input!(attr as AttributeArgs)
        .into_iter()
        .filter_map(|nm| match nm {
            // Only select certain tokens
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                eq_token: _,
                lit: Lit::Str(lstr),
            })) => Some(
                // Convert the segment of the path to a string
                (
                    path.segments
                        .into_iter()
                        .map(|seg| seg.ident.to_string())
                        .collect::<Vec<String>>()
                        .swap_remove(0),
                    // get the raw value of the LitStr
                    lstr.value(),
                ),
            ),
            _ => None,
        })
        // convert the (String, LitStr) tuple to a custom type which only accepts certain key/value pairs
        .map(|(k, v)| FromZbusMessageParam::from(k, v))
        .collect();

    let body_type = match args_parsed
        .get(0)
        .expect("There must be at least one argument to the macro.")
    {
        FromZbusMessageParam::Body(body_type) => body_type,
        _ => panic!("The body parameter must be set first, and must be a type."),
    };
		// if the member is set explicitly, use it, otherwise, use the struct name.
    let member = match args_parsed
        .get(1)
    {
        Some(FromZbusMessageParam::Member(member_str)) => member_str,
				_ => name_string.strip_suffix("Event").unwrap(),
    };

    // Generate the expanded code
    let expanded = quote! {
        #item_struct
        impl TryFrom<Arc<Message>> for  #name {
            type Error = AtspiError;

            fn try_from(message: Arc<Message>) -> Result<Self, Self::Error> {
                let message_member: MemberName = message
                    .member()
                    .ok_or(AtspiError::MemberMatch("message w/o member".to_string()))?;

                let member = MemberName::from_static_str(#member)?;

                if message_member != member {
                    let error = format!("message member: {:?} != member: {:?}", message_member, member);
                    return Err(AtspiError::MemberMatch(error));
                };
                let body: #body_type = message.body()?;
                Ok(Self { message, body })
            }
        }

    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}

#[proc_macro_derive(GenericEvent)]
pub fn generic_event(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    // Extract the name of the struct
    let name = &ident;

    // Generate the expanded code
    let expanded = quote! {
            impl GenericEvent for #name {
                    /// Bus message.
                    #[must_use]
                    fn message(&self) -> &Arc<Message> {
                            &self.message
                    }

                    /// For now this returns the full interface name because the lifetimes in [`zbus_names`][zbus::names] are
                    /// wrong such that the `&str` you can get from a
                    /// [`zbus_names::InterfaceName`][zbus::names::InterfaceName] is tied to the lifetime of that
                    /// name, not to the lifetime of the message as it should be. In future, this will return only
                    /// the last component of the interface name (I.E. "Object" from
                    /// "org.a11y.atspi.Event.Object").
                    #[must_use]
                    fn interface(&self) -> Option<InterfaceName<'_>> {
                            self.message.interface()
                    }

                    /// Identifies this event's interface member name.
                    #[must_use]
                    fn member(&self) -> Option<MemberName<'_>> {
                            self.message.member()
                    }

                    /// The object path to the object where the signal is emitted from.
                    #[must_use]
                    fn path(&self) -> std::option::Option<zbus::zvariant::OwnedObjectPath> {
                            Some(OwnedObjectPath::from(self.message.path().unwrap()))
                    }

                    /// Identifies the `sender` of the event.
                    /// # Errors
                    /// - when deserializeing the header failed, or
                    /// - When `zbus::get_field!` finds that 'sender' is an invalid field.
                    fn sender(&self) -> Result<Option<zbus::names::UniqueName>, crate::AtspiError> {
                            Ok(self.message.header()?.sender()?.cloned())
                    }
                }
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}
