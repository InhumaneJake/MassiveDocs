RPCs
=====================

In the synchronization section, we learned about how quickly changing variables such as position are kept in sync by sending unreliable messages in quick intervals. What about everything else?

MassiveNet supports RPCs (Remote Procedure Calls). By adding the [NetRPC] attribute to a method inside of a MonoBehaviour, MassiveNet will be aware that the method supports being called as a remote procedure.

A simple example of an RPC being sent from within script attached to a NetView GameObject::

 public class SomeClientScript : MonoBehaviour {
 
     NetView view;
     
     void Start() {
         view = GetComponent<NetView>;
         SayHello();
     }
     
     // Makes the server print "Hello, Server!" three times.
     void SayHello() {
         view.SendReliable("HearMessage", RPCTarget.Server, "Hello, Server", 3);
     }
 }

Server side::

 public class SomeServerScript : MonoBehaviour {
     
     [NetRPC]
     void HearMessage(string message, int printCount) {
         for (int i = printCount; i > 0; i--) {
             Debug.Log(message);
         }
     }
 }

The server will reject any RPC messages coming from a connection that is not authorized to speak to the NetView. Only controllers (Owners), servers, and peers may send messages to a NetView.


But what if it doesn't make sense to send the RPC to a NetView? What if the message isn't related to GameObjects at all?
Maybe we want to send a message to the server to inform that we would like to start the logout process. 

In cases where a message needs to be sent without being associated with a NetView, you can send them directly through the NetSocket. In the example project, you might attach a script to the Client or Server object that sends and receives RPCs without any associated NetView.

For example::

 public class SomeViewlessClientScript : MonoBehaviour {
 
     NetSocket socket;
     
     void Start() {
         socket = GetComponent<NetSocket>();
         socket.Events.OnConnectedToServer += SayHowdy;
     }
     
     void SayHowdy(NetConnection server) {
         socket.Send("ReceiveGreeting", server, "Howdy, Server!");
     }
 }
 
 public class SomeViewlessServerScript : MonoBehaviour {
     NetSocket socket;
     
     void Start() {
         socket = GetComponent<NetSocket>();
         socket.RegisterRpcListener(this);
     }
     
     [NetRPC]
     void ReceiveGreeting(string greeting, NetConnection sender) {
         Debug.Log("Received a greeting from " + sender.Endpoint + ": " + greeting);
     }
 }


You may have noticed something peculiar above, and it's not the southern accent. In the server script, you'll find that there's an extra parameter defined for the ReceiveGreeting RPC. The Client doesn't send a NetConnection (and can't), so how and why is NetConnection a parameter, and why doesn't it cause a problem?

In MassiveNet, NetConnection is a special case. Since it doesn't make sense to send a NetConnection, the opportunity was taken to allow a NetConnection parameter to be added to an RPC so that you can identify where the RPC came from. 
