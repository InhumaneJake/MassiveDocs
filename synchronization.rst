Synchronization
=====================

Synchronization in MassiveNet refers to the periodic transmission of changes in a NetView's state to all in-scope connections. 

The most common example of this in real-time multiplayer games is synchronization of position. Most games have objects that move non-deterministically, that is, their position changes in a way that cannot be determined given the information that is currently known. Given that, there must be a method to keep everyone up to date without using too much bandwidth and computational power.


MassiveNet's solution to synchronization is NetSync, which is an event that is fired by a NetView at intervals defined by the SyncsPerSecond parameter of the ViewManager component. Here's an example of a MonoBehaviour making use of the NetSync event to write position synchronization to the provided stream::

  private NetView view;
  
  void Awake() {
      view = GetComponent<NetView>();
      view.OnWriteSync += WriteSync;
  }
  
  RpcTarget WriteSync(NetStream syncStream) {
      syncStream.WriteVector3(transform.position);
      syncStream.WriteFloat(transform.rotation.eulerAngles.y);
      return RpcTarget.NonControllers;
  }

The receiving end might look like this::

  private NetView view;

  void Awake() {
      view = GetComponent<NetView>();
      view.OnReadSync += ReadSync;
  }
  
  void ReadSync(NetStream syncStream) {
      transform.position = syncStream.ReadVector3();
      float yRot = syncStream.ReadFloat();
      transform.rotation = Quaternion.Euler(0, yRot, 0);
  }
