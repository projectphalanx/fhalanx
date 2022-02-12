# fhalanx

Issue #1
220208 Trying to figure out how to read the return from OrdersOrderGet. It is a type Option<Order>
 - Openbrush forum  =>  Told me to study polkadot api doc here https://polkadot.js.org/docs/api/start/typescript
                            - not sure how to apply that.
                    => Told me to do 
                        await fromSigner(orders.contract, alice.address).query.ordersOrderGet(alice.address).then((queryOut) => console.log("Test:" + queryOut.output.toHuman()))
       Returns
       2022-02-08 15:30:05        REGISTRY: Unable to resolve type PhalanxOrdersOrder, it will fail on construction
Test:null

Issue #2
220207 Tried to add ownable to the orders traits. Getting compile errors. 
 - OpenBrush forum => Tried to apply suggestion below. Still compile errors
    Yes, it is possible. For that you need to add super trait OwnableStorage for your trait to be able to use only_owner modifier=)

    #[brush::trait_definition]
    pub trait SomeTrait: OwnableStorage {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn some_method(&mut self) {
            ...
        }
    }

