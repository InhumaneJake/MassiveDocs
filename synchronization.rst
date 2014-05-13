Synchronization
=====================

Synchronization in MassiveNet refers to the periodic transmission of changes in a NetView's state to all in-scope connections. 

The most common example of this in real-time multiplayer games is synchronization of position. Most games have objects that move non-deterministically, that is, their position changes in a way that cannot be determined given the information that is currently known. Given that, there must be a method to keep everyone up to date without using too much bandwidth and computational power.


MassiveNet's solution to synchronization is NetSync, which is an event that is fired by a NetView at intervals defined by the SyncsPerSecond parameter of the ViewManager component. Here's an example of a Monobehaviour making use of the NetSync event to write position synchronization to the provided stream::

  void Start()
  {
    netView = GetComponent<NetView>();
    netView.OnNetSync += NetSync;
    controller = GetComponent<CharacterController>();
  }
  
  void NetSync(NetStream syncStream)
  {
    syncStream.WriteVector3(rigidbody.position);
    syncStream.WriteQuaternion(rigidbody.rotation);
    syncStream.WriteVector2(new Vector2(inputX, inputZ));
    netView.SendSync("PlayerMove", RpcTarget.Server);
  }

The receiving end might look like this::

  [NetRPC]
  void PlayerMove(Vector3 position, Quaternion rotation, Vector2 velocity)
  {
    rigidbody.position = position;
    rigidbody.rotation = rotation;
    lastVel = velocity;
  }
