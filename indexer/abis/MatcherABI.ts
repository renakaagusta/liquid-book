export const MatcherABI = [
    {
        "inputs": [
            { "internalType": "address", "name": "bitmap_manager_address", "type": "address" },
            { "internalType": "address", "name": "order_manager_address", "type": "address" }
        ],
        "name": "initialize",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "tuple[]",
                "name": "valid_orders",
                "type": "tuple[]",
                "components": [
                    { "internalType": "uint256", "name": "field1", "type": "uint256" },
                    { "internalType": "uint256", "name": "field2", "type": "uint256" },
                    { "internalType": "uint256", "name": "field3", "type": "uint256" }
                ]
            },
            { "internalType": "uint256", "name": "incoming_order_volume", "type": "uint256" },
            { "internalType": "uint256", "name": "tick_value", "type": "uint256" },
            { "internalType": "uint256", "name": "tick_volume", "type": "uint256" }
        ],
        "name": "execute",
        "outputs": [
            { "internalType": "uint256", "name": "", "type": "uint256" }
        ],
        "stateMutability": "nonpayable",
        "type": "function"
    }
] as const;