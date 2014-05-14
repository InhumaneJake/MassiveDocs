Getting Started
=================

MassiveNet comes with an example project designed to introduce you to the basics of using the library. There are three separate projects nested under the Massive > Examples > NetSimple folder. You'll find Client, Lobby, and Server folders, each containing their own scripts, prefabs, and scenes. 

The main scripts to pay attention to are the ClientModel, ServerModel, and LobbyModel scripts. They are responsible for NetSocket configuration and startup as well as basic logic for handling connections. 

You'll also want to take a look a the scripts responsible for position synchronization, which are NetController and PlayerSync in the Client > Scripts folder, and the PlayerCreator and AiCreator scripts in the Servers > Scripts folder.


You'll notice in the LobbyModel that two Zones are created. It is recommended you read up on Zones to understand their purpose, but for now, let's just say that they are Server configurations. This particular example requires two servers to connect to the Lobby before a client can connect. This is because two Zones are created in the Start method of the LobbyModel class. 

To run the example project, build each scene separately, then start the lobby, two servers, and one or more clients. 

