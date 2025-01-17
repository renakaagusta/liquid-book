export const TickManagerABI =
    [
        {
            "anonymous": false,
            "inputs": [
                { "indexed": true, "internalType": "uint256", "name": "tick", "type": "uint256" },
                { "indexed": true, "internalType": "bool", "name": "is_buy", "type": "bool" },
                { "indexed": true, "internalType": "uint256", "name": "volume", "type": "uint256" },
                { "indexed": false, "internalType": "bool", "name": "is_existing_order", "type": "bool" }
            ],
            "name": "SetTickData",
            "type": "event"
        },
        {
            "inputs": [
                { "internalType": "address", "name": "engine_address", "type": "address" },
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
                { "internalType": "uint256", "name": "tick", "type": "uint256" },
                { "internalType": "uint256", "name": "volume", "type": "uint256" },
                { "internalType": "bool", "name": "is_buy", "type": "bool" },
                { "internalType": "bool", "name": "is_existing_order", "type": "bool" }
            ],
            "name": "setTickData",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                { "internalType": "uint256", "name": "tick", "type": "uint256" }
            ],
            "name": "getTickData",
            "outputs": [
                { "internalType": "uint256", "name": "", "type": "uint256" },
                { "internalType": "uint256", "name": "", "type": "uint256" },
                { "internalType": "uint256", "name": "", "type": "uint256" },
                { "internalType": "bool", "name": "", "type": "bool" }
            ],
            "stateMutability": "view",
            "type": "function"
        }
    ] as const;