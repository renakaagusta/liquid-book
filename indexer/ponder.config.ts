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
      address: "0x4307c7da3fa5a45a8e280bad6bbe01b57a3b4e4e",
      startBlock: 114371569,
    },
    TickManager: {
      network: "arbitrumSepolia",
      abi: TickManagerABI,
      address: "0xe61baf90143c169632d198809a44a20357d2c7d2",
      startBlock: 114371602,
    },
    OrderManager: {
      network: "arbitrumSepolia",
      abi: OrderManagerABI,
      address: "0x0e10b18ac7eec8fb3d85f3e532dfae381c7bca95",
      startBlock: 114371533,
    },
    BitmapManager: {
      network: "arbitrumSepolia",
      abi: BitmapManagerABI,
      address: "0xd0061c14d056d4ee06d8b048f3a7bc0304396c31",
      startBlock: 114371585,
    },
    Matcher: {
      network: "arbitrumSepolia",
      abi: MatcherABI,
      address: "0x993c639ddadc5c427281c1adbc2dfef122be98cb",
      startBlock: 114371555,
    },
  },
});
