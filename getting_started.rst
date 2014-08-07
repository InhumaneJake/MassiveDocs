Getting Started
=================

MassiveNet comes with an example project designed to introduce you to the basics of using the library. There are two separate projects nested under the MassiveNet > Examples > NetSimple folder. You'll find Client and Server folders, each containing their own scripts, prefabs, and scenes. 

The main scripts to pay attention to are the ClientModel and ServerModel scripts. They are responsible for NetSocket configuration and startup as well as basic logic for handling connections. 

You'll also want to take a look a the scripts responsible for position synchronization, which are NetController and PlayerSync in the Client > Scripts folder, and the PlayerCreator and AiCreator scripts in the Servers > Scripts folder.


You'll notice in the ServerModel that two Zones are created. It is recommended you read up on Zones to understand their purpose, but for now, let's just say that they are Server configurations. Because two Zones are configured in this example project, two servers are required to host the game world.

To run the example project, build each scene seperately, then start two servers and one or more clients. 

Here is a breakdown of how this example works:

  1. The first Server is started. Since the port is not incremented and there are no peer addresses defined in the Peer list, the ServerModel determines that it will act as the ZoneManager and configures two Zones. The ZoneManager assigns Server 1 to the first Zone.
  
  2. The second Server is started. Since the initial port is in use, the ServerModel determines it should attempt to connect to a peer listening on the initial port.
  
  3. Since Server 1 is acting as the ZoneManager, Server 1 will assign Server 2 to the remaining unassigned Zone.
  
  4. Now that all Zones are assigned, the game world is ready for clients.
  
  5. When a Client connects to Server 1, the Server informs the Client that it will be sending numeric assignments for RPC method names. This is because both Servers are configured to be Protocol Authorities. A Protocol Authority is reponsible for assigning automatically generated numeric Ids to RPC method names. The Server first sends all of the assignments for its own NetRPC methods, and then the Client will request assignments for any NetRPC methods unique to the Client.
  
  7. Once RPC IDs have been resolved, the connection is complete. Server 1's ZoneServer component then sends an RPC to the Client's ZoneClient component, informing it that it must connect to Server 2.
  
  8. If the Client has successfully connected to Server 2, it will inform Server 1 that Zone setup is complete.
  
  9. Upon successful Zone setup, the Client signals to Server 1 that it would like to be spawned. Server 1 will then instantiate a Player object for the Client via the ViewManager component.
  
  11. The Client can now freely move it's Player object around the game world. When the Client's Player gets close enough to the Sphere zone, the Client's Player object will be seamlessly handed off to Server 2, which is responsible for the area around the Sphere. The Client can see the AI objects from both servers if it is in range, but only one of the two Servers will handle the Client's Player object at any one time.
