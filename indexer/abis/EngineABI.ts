export const EngineABI = [
    {
        "inputs": [
            { "internalType": "address", "name": "tick_manager_address", "type": "address" },
            { "internalType": "address", "name": "order_manager_address", "type": "address" },
            { "internalType": "address", "name": "bitmap_manager_address", "type": "address" },
            { "internalType": "address", "name": "matcher_manager_address", "type": "address" }
        ],
        "name": "initialize",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            { "internalType": "uint256", "name": "incoming_order_tick", "type": "uint256" },
            { "internalType": "uint256", "name": "incoming_order_volume", "type": "uint256" },
            { "internalType": "address", "name": "incoming_order_user", "type": "address" },
            { "internalType": "bool", "name": "incoming_order_is_buy", "type": "bool" },
            { "internalType": "bool", "name": "incoming_order_is_market", "type": "bool" }
        ],
        "name": "placeOrder",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "anonymous": false,
        "inputs": [
            { "indexed": true, "internalType": "address", "name": "user", "type": "address" },
            { "indexed": true, "internalType": "uint256", "name": "tick", "type": "uint256" },
            { "indexed": true, "internalType": "bool", "name": "is_buy", "type": "bool" },
            { "indexed": false, "internalType": "uint256", "name": "volume", "type": "uint256" }
        ],
        "name": "PlaceOrder",
        "type": "event"
    }
] as const;