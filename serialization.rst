Serialization
=====================

Serialization (and deserialization) is a very important concept in computer science. It encompasses varying methodologies for communication and storage of data between disparate systems. Put simply, it can be defined as a method of turning your objects/data into zeroes and ones so that it may be sent across the network, and then turning them back into useable objects/data on the receiving end.

There are many different ways to serialize and deserialize data, each with its own set of strengths and weaknesses. For the needs of real-time networked games, the process of serializing and deserializing must be both quick and compact.

By default, MassiveNet provides serialization and deserialization of the following types:

* bool
* byte
* short
* ushort
* int
* uint
* float
* double
* string
* Vector2
* Vector3
* Quaternion


This is not the end of supported types, however. MassiveNet allows the use of custom (de)serializers, which are simply separate methods for both serialization and deserialization of a custom type.


For example, lets say there is a class called PlayerData. Here's what it might look like::

  public class PlayerData {
  
    string name;
    int hp;
    Vector3 position;
    
    public static void SerializePlayerData(NetStream stream, object instance) {
      PlayerData data = (PlayerData)instance;
      stream.WriteString(data.name);
      stream.WriteInt(data.hp);
      stream.WriteVector3(data.position);
    }
    
    public static object DeserializePlayerData(NetStream stream) {
      PlayerData data = new PlayerData();
      data.name = stream.ReadString();
      data.hp = stream.ReadInt();
      data.position = stream.ReadVector3();
      return data;
    }
    
  }


Now, all that is needed is to register the static methods responsible for serializing and deserializing PlayerData. To do this, all you must do is call the following from anywhere::

  NetSerializer.Add<PlayerData>(PlayerData.SerializePlayerData, PlayerData.DeserializePlayerData);



