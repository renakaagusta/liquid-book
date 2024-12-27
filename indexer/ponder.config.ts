import { createConfig } from "ponder";
import { http } from "viem";

import { EngineABI } from "./abis/EngineABI";
import { OrderManagerABI } from "./abis/OrderManagerABI";
import { TickManagerABI } from "./abis/TickManagerABI";
import { BitmapManagerABI } from "./abis/BitmapManagerABI";
import { MatcherABI } from "./abis/MatcherABI";

export default createConfig({
  networks: {
    arbitrumSepolia: {
      chainId: 421614,
      transport: http(process.env.PONDER_RPC_URL_1),
    },
  },
  contracts: {
    Engine: {
      network: "arbitrumSepolia",
      abi: EngineABI,
      address: "0xf7B97478528a26a2BCe77d2B34A5BB078aDbaaf3",
      startBlock: 111059028,
    },
    TickManager: {
      network: "arbitrumSepolia",
      abi: TickManagerABI,
      address: "0x36C6F2442ADc8993abd0E97F47e5711fC30633B5",
      startBlock: 111059134,
    },
    OrderManager: {
      network: "arbitrumSepolia",
      abi: OrderManagerABI,
      address: "0x4149e31e3498032030cb26b872b0D4FeC9734877",
      startBlock: 111059147,
    },
    BitmapManager: {
      network: "arbitrumSepolia",
      abi: BitmapManagerABI,
      address: "0xf5277Eb468001416ea15557096F7D0fF28CBfD94",
      startBlock: 111059160,
    },
    Matcher: {
      network: "arbitrumSepolia",
      abi: MatcherABI,
      address: "0xE6Ae209eA8974ce6A1996C10471f14bdc8ddBd41",
      startBlock: 111059116,
    },
  },
});
