Scope
=====================

Scope is a way of dynamically defining how often a connected client should receive synchronization information about a NetView, or whether or not they should even receive information at all.

In large game worlds where there are many clients and
NetViews, sending all information to all clients at all times could be prohibitively expensive, both for bandwidth and processing power. To provide high efficiency, scoping must be utilized. Fortunately, MassiveNet provides this functionality right out of the box.

In a traditional multiplayer game, it is assumed that a connecting client needs to instantiate (or spawn) every network object in the game as well as receive updates for these objects at all times. The implication is that all network objects are in scope and you must explicitly override this default behavior to provide scoping functionality.

With MassiveNet, all objects are implicitly out of scope for a connection. For a connection to receive instantiation information and synchronization updates for a NetView, you must set that NetView as in-scope for the connection. When a game server is utilizing the ScopeManager component, this is handled automatically. The ScopeManager incrementally updates the scope of each NetView in relation to each connected client in order to determine which NetViews are in scope for the connection.

This isn't the end of the importance of scoping, however. Just like how 3D models can have different levels of detail based on the camera's distance from them, client connections can have different levels of scope for NetViews based on their distance from the client's in-game representation. This is sometimes referred to as network LOD (Level of Detail) due to its parallels with traditional application of LOD for reducing complexity of 3D models.

MassiveNet's ScopeManager is able to set three different levels of scope for in-scope NetViews. These three levels correspond to the distance between the client and the NetView's game object. The first scope level means the connection will receive every synchronization message, the second, every other synchronization message, and the third, every fourth synchronization message.
