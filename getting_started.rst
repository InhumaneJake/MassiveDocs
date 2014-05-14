Getting Started
=================

MassiveNet comes with an example project designed to introduce you to the basics of using the library. There are three separate projects nested under the Massive > Examples > NetSimple folder. You'll find Client, Lobby, and Server folders, each containing their own scripts, prefabs, and scenes. 

The main scripts to pay attention to are the ClientModel, ServerModel, and LobbyModel scripts. They are responsible for NetSocket configuration and startup as well as basic logic for handling connections. 

You'll also want to take a look a the scripts responsible for position synchronization, which are NetController and PlayerSync in the Client > Scripts folder, and the PlayerCreator and AiCreator scripts in the Servers > Scripts folder.


You'll notice in the LobbyModel that two Zones are created. It is recommended you read up on Zones to understand their purpose, but for now, let's just say that they are Server configurations. This particular example requires two servers to connect to the Lobby before a client can connect. This is because two Zones are created in the Start method of the LobbyModel class. 

To run the example project, build each scene separately, then start the lobby, two servers, and one or more clients. 

Here is a breakdown of how this example works:

  1. The Lobby is started. In the LobbyModel Start method, the NetSocket and Zones are created and configured.
  
  2. Two Servers are started. The Servers generate a random port between 17100 and 18000 to listen on, configure the       NetSocket with settings appropriate for a Server, and start the NetSocket. After that, the Servers connect to the Lobby   using the address provided in the LobbyAddress string.
  
  3. After the Servers connect to the Lobby, the Lobby first provides numeric assignments to all of the RPC method names   that the Servers possess. This happens because the Lobby is configured as the RPC Definition Authority. There must always be an RPC Definition Authority to assign numeric IDs. MassiveNet assigns numeric IDs to RPCs to reduce the bandwidth requirements of sending RPCs.
  
  4. Once the Servers receive RPC ID assignments, they are each assigned to a Zone. In the case of this example, the LobbyModel created two zones, Cube and Sphere, so each Server will be assigned to one of these. 
  
  5. After the Servers have processed their assignments, they signal to the Lobby that the assignment is successful. Clients can now connect to the game world.
  
  6. When a Client connects to the Lobby, the Lobby first sends RPC ID assignments to the Client.
  
  7. Once RPC IDs have been resolved, the Client connection is signaled as completed and the Client sends an RPC to the Lobby signaling that it wants to connect to the game world.
  
  8. Once the Lobby receives the RPC from the Client, it first checks that there is a Server assigned to each Zone. Then, the Lobby informs the Server assigned to the Cube zone that a Client is connecting and that it should spawn a Player object for it.
  
  9. Once the Cube Server sends confirmation to the Lobby, the Lobby sends an RPC which tells the Client to connect to both the Cube and the Sphere Servers.
  
  10. Once the Client connects to the Cube server, a Player object is spawned for that Client.
  
  11. The Client can now freely move around the game world. When the Client gets close enough to the Sphere zone, the Client's Player object will be seamlessly handed off to the Sphere. The Client can see the AI objects from both servers if it is in range, but only one of the two Servers will handle the Client's Player object at any one time.
