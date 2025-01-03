export const MatcherABI = [
    {
        "inputs": [
            {
                "internalType": "address",
                "name": "bitmap_manager_address",
                "type": "address"
            },
            {
                "internalType": "address",
                "name": "order_manager_address",
                "type": "address"
            }
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
                "components": [
                    {
                        "internalType": "int128",
                        "name": "",
                        "type": "int128"
                    },
                    {
                        "internalType": "uint256",
                        "name": "",
                        "type": "uint256"
                    },
                    {
                        "internalType": "uint256",
                        "name": "",
                        "type": "uint256"
                    }
                ],
                "name": "valid_orders",
                "type": "tuple[]"
            },
            {
                "internalType": "uint256",
                "name": "incoming_order_volume",
                "type": "uint256"
            },
            {
                "internalType": "int128",
                "name": "tick_value",
                "type": "int128"
            },
            {
                "internalType": "uint256",
                "name": "tick_volume",
                "type": "uint256"
            }
        ],
        "name": "execute",
        "outputs": [
            {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
            }
        ],
        "stateMutability": "nonpayable",
        "type": "function"
    }
] as const;