/// Any boxed type that may or may not contain data.
/// They are fed to your applications `update` method to tell it how and what to update.
///
/// Typically, you will use the `downcast_ref` method on your messages to determine the type of the message,
/// and extract the data from them if present.
///
/// # Example
///
/// ```
/// // the type of your message
/// struct HttpResponse(String);
///
/// // the boxed message itself
/// let http_response_message = Box::new(HttpResponse("Hello World".to_string()));
///
/// // determining the type of your message, and extracting the response
/// if let Some(res) = http_response_message.downcast_ref::<HttpResponse>() {
///     // do something with the response
///     // for example, setting it in the model to be rendered
///     model.response = Some(res);
/// }
/// ```
pub type Message = Box<dyn std::any::Any + Send>;
