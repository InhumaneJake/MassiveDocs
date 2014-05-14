Views
=====================

In MassiveNet, like in Unity Networking, a NetView is representative of a communication channel reserved for a network object. A NetView provides a unique network identifier, concept of ownership, and facilities for synchronizing the state of the object. A NetView component must be attached to each object which has a state that must be synchronized. 

NetViews are managed by the ViewManager component. A ViewManager component should be attached to the same GameObject as the NetSocket component for which it operates. While a ViewManager is required when utilizing NetViews, it is not a requirement for using MassiveNet. As shown in the NetSimple example, the Lobby does not make use of a ViewManager at all.




