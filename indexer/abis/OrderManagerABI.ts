export const OrderManagerABI = [
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "address",
          "name": "user",
          "type": "address"
        },
        {
          "indexed": true,
          "internalType": "int128",
          "name": "tick",
          "type": "int128"
        },
        {
          "indexed": true,
          "internalType": "uint256",
          "name": "order_index",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "bool",
          "name": "is_buy",
          "type": "bool"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "volume",
          "type": "uint256"
        }
      ],
      "name": "InsertOrder",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "int128",
          "name": "tick",
          "type": "int128"
        },
        {
          "indexed": true,
          "internalType": "uint256",
          "name": "order_index",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "volume",
          "type": "uint256"
        }
      ],
      "name": "UpdateOrder",
      "type": "event"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "engine_address",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "bitmap_manager_address",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "tick_manager_address",
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
          "internalType": "int128",
          "name": "tick",
          "type": "int128"
        },
        {
          "internalType": "uint256",
          "name": "volume",
          "type": "uint256"
        },
        {
          "internalType": "address",
          "name": "user",
          "type": "address"
        },
        {
          "internalType": "bool",
          "name": "is_buy",
          "type": "bool"
        }
      ],
      "name": "insertOrder",
      "outputs": [
        {
          "internalType": "uint256",
          "name": "",
          "type": "uint256"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "int128",
          "name": "tick",
          "type": "int128"
        },
        {
          "internalType": "uint256",
          "name": "volume",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "order_index",
          "type": "uint256"
        }
      ],
      "name": "updateOrder",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "int128",
          "name": "tick",
          "type": "int128"
        },
        {
          "internalType": "uint256",
          "name": "order_index",
          "type": "uint256"
        }
      ],
      "name": "readOrder",
      "outputs": [
        {
          "internalType": "address",
          "name": "",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "",
          "type": "uint256"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "int128",
          "name": "tick",
          "type": "int128"
        },
        {
          "internalType": "uint256",
          "name": "order_index",
          "type": "uint256"
        },
        {
          "internalType": "address",
          "name": "user",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "volume",
          "type": "uint256"
        }
      ],
      "name": "writeOrder",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "int128",
          "name": "tick",
          "type": "int128"
        },
        {
          "internalType": "uint256",
          "name": "order_index",
          "type": "uint256"
        }
      ],
      "name": "deleteOrder",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "int128",
          "name": "tick",
          "type": "int128"
        },
        {
          "internalType": "uint256",
          "name": "order_index",
          "type": "uint256"
        }
      ],
      "name": "encodeOrderKey",
      "outputs": [
        {
          "internalType": "uint8[]",
          "name": "",
          "type": "uint8[]"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "user",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "volume",
          "type": "uint256"
        }
      ],
      "name": "encodeOrderData",
      "outputs": [
        {
          "internalType": "uint8[32]",
          "name": "",
          "type": "uint8[32]"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "uint8[]",
          "name": "encoded",
          "type": "uint8[]"
        }
      ],
      "name": "decodeOrderData",
      "outputs": [
        {
          "internalType": "address",
          "name": "",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "",
          "type": "uint256"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    }
  ] as const;