Instantiation
=====================

To create a GameObject that will send and receive network messages, you must first create a prefab for it. The prefab must have a NetView component attached.

In MassiveNet, there are four identifiers for prefabs: Creator, Owner, Peer, and Proxy. Creator is the prefab used by the creator/server of the NetView, Owner is for those authorized to communicate/control the NetView, such as a client, Peer is for connections that are a Peer to the Creator, and Proxy is for those who need to observe the NetView but are not allowed to send communications to it.

These identifiers are added to the end of a prefab name, preceded by a "@". For example: "Player@Creator" or "Player@Owner".

To instantiate a NetView prefab, you must call NetViewManager.CreateView and provide the basic necessary information. If you are spawning a NetView that will be controlled by another connection, you need to provide the connection, the group number (0 is the default group), and the prefab root. The prefab root is everything before the "@" in the prefab name.

For example, spawning a Player object on the Server that will be controlled by a Client would look similar to::

  public class Spawner {
  
    NetViewManager viewManager;
    
    void Awake() {
        viewManager = GetComponent<NetViewManager>();
    }
    
    // Spawns "Player@Creator" locally and "Player@Owner" on connection
    // Assigned to default group 0
    public void SpawnPlayerFor(NetConnection connection) {
        viewManager.CreateView(connection, 0, "Player");
    }


Here's what happens when CreateView is called:

1) The proper local prefab name is determined based on who the controller (if any) and server are.

2) The prefab with the generated prefab name (e.g., "Player@Creator") is instantiated. If the ViewManager.InstantiatePrefab delegate is assigned (for use by an object pooling system, for example), the delegate will be called. Otherwise, the GameObject will be instantiated normally. If the resulting GameObject does not have a NetView, the GameObject will be destroyed and an exception will be thrown.

3) The OnNetViewCreated event will be triggered.

4) As a result of the event, the ScopeManager (if any) will perform an immediate scope calculation for the new NetView to determine which connections are in-scope. Every in-scope connection will receive the command and data to instantiate the new View.

5) When the ScopeManager tells the NetView to send instantiation data to an in-scope connection, the NetView determines the relationship level of the connection to decide which instantiation data collection event should be fired. There is an instantiation event for each type of relationship (Creator, Owner, Peer, and Proxy). For security and efficiency reasons, you wouldn't want to send the entire contents of a Player's inventory to every other client (Proxy), for example, but you probably do want to send what they have equipped for visual reasons. The events are aptly named: OnWriteOwnerData, OnWriteCreatorData, OnWriteProxyData, and OnWritePeerData

6) The NetView calls the relevant event to gather the necessary data to instantiate itself for the connection and then sends the command and its associated data.

...

On the receiving end of an Instantiate command:

7) Steps 1 through 3 happen on the receiving end of each connection that receives the instantiate command. 

8) Since the connection is on the receiving end of the instantiate command, the NetView will take the instantiation data (NetStream) it received with the command and trigger OnReadInstantiateData. The data in the NetStream can be used to perform the initial setup. One of the most common uses for this initialization data is to set the position of the NetView.
