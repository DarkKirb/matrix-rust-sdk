initSidebarItems({"attr":[["async_trait",""]],"enum":[["Error","Internal representation of errors."],["HttpError","An HTTP error, representing either a connection error or an error while converting the raw HTTP response into a Matrix response."],["LoopCtrl","Enum controlling if a loop running callbacks should continue or abort."],["RoomType","Enum keeping track in which state the room is, e.g. if our own user is joined, invited, or has left the room."],["StoreError","State store specific error type."]],"mod":[["config","Configuration to change the behaviour of the `Client`."],["deserialized_responses",""],["encryption","End-to-end encryption related types"],["event_handler","Types and traits related for event handlers. For usage, see [`Client::register_event_handler`]."],["executor","Abstraction over an executor so we can spawn tasks under WASM the same way we do usually."],["instant",""],["locks",""],["media","Common types for media content."],["room","High-level room API"],["util",""]],"struct":[["BaseRoom","The underlying room data structure collecting state for joined, left and invited rooms."],["BaseRoomMember","A member of a room."],["Client","An async/await enabled Matrix client."],["RoomInfo","The underlying pure data structure for joined and left rooms."],["RoomMember","The high-level `RoomMember` representation"],["Session","A user session, containing an access token and information about the associated user account."],["StateChanges","Store state changes and pass them to the StateStore."]],"trait":[["AsyncTraitDeps","Super trait that is used for our store traits, this trait will differ if it’s used on WASM. WASM targets will not require `Send` and `Sync` to have implemented, while other targets will."],["HttpSend","Abstraction around the http layer. The allows implementors to use different http libraries."]],"type":[["HttpResult","Result type of a pure HTTP request."],["Result","Result type of the matrix-sdk."]]});