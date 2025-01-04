export const BitmapManagerABI =
[
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "int128",
          "name": "tick",
          "type": "int128"
        }
      ],
      "name": "SetCurrentTick",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "int32",
          "name": "tick",
          "type": "int32"
        }
      ],
      "name": "FlipTick",
      "type": "event"
    },
    {
      "inputs": [
        {
          "internalType": "int32",
          "name": "tick",
          "type": "int32"
        }
      ],
      "name": "position",
      "outputs": [
        {
          "internalType": "int16",
          "name": "",
          "type": "int16"
        },
        {
          "internalType": "uint8",
          "name": "",
          "type": "uint8"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "getCurrentTick",
      "outputs": [
        {
          "internalType": "int128",
          "name": "",
          "type": "int128"
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
        }
      ],
      "name": "setCurrentTick",
      "outputs": [
        {
          "internalType": "int128",
          "name": "",
          "type": "int128"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "bool",
          "name": "is_buy",
          "type": "bool"
        }
      ],
      "name": "topNBestTicks",
      "outputs": [
        {
          "internalType": "int128[]",
          "name": "",
          "type": "int128[]"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "int32",
          "name": "tick",
          "type": "int32"
        }
      ],
      "name": "flip",
      "outputs": [
        {
          "internalType": "int16",
          "name": "",
          "type": "int16"
        },
        {
          "internalType": "uint8",
          "name": "",
          "type": "uint8"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "int16",
          "name": "index",
          "type": "int16"
        }
      ],
      "name": "getBitmap",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "int32",
          "name": "tick",
          "type": "int32"
        },
        {
          "internalType": "bool",
          "name": "lte",
          "type": "bool"
        }
      ],
      "name": "nextTick",
      "outputs": [
        {
          "internalType": "int32",
          "name": "",
          "type": "int32"
        },
        {
          "internalType": "bool",
          "name": "",
          "type": "bool"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    }
  ] as const;