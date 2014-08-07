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
