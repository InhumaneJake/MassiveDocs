Zones
=====================

Zones are a way of representing an area of the game world in terms relevant to game servers. When you break up a game world into Zones, you define an area of responsibility for a server. On top of this, when you define a Zone, you make it clear that a server must exist for that area of the game.

When players (or NPCs) move throughout the game world, the NetworkView that represents them will move across zone boundaries. When that occurs, the server that is currently responsible for the NetworkView changes. This process is called a handoff.

In a classic server/client architecture, a client is only connected to one game server at a time. Sure, they may also be connected to a master server, a chat server, etc., but the client is usually only connected to one server that represents the simulation of the physical game space.

With the MassiveNet architecture, a client is often connected to more than one game server at a time. When a client nears the boundaries of the Zone it is currently in, it needs to be able to observe network objects (NetViews) that are in those nearby Zones. Likewise, a client will need to quickly and seamlessly be able to switch its communication target to a new server when it crosses the Zone boundary. 

When a game server possessing a ZoneListener component connects to another server with a ZoneMaster component, the ZoneMaster will assign the ZoneListener to an unassigned Zone if one is available. When this happens, the ZoneMaster will send all of the data necessary for the game server to fill its role in the game world.

One of a game server's responsibilities is checking the location of each NetView it is currently responsible for and handing off that responsibility to a peer Zone when necessary. The criteria for this responsibility switch, or handoff, are defined within the parameters of the Zone class. Using a combination of the server's own Zone parameters and that of its peers, a server can hand off the responsibility for a NetView to the appropriate peer. Once the peer has taken responsibility, the new server of that NetView will notify the client(s) (if any) that control that NetView that it's time to switch communication target.
