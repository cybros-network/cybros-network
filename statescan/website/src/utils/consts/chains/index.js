import { ReactComponent as CybrosPrimalIcon } from "../../../components/icons/cybros-primal.svg";

const cybrosPrimal = {
  name: "Cybros Primal",
  icon: <CybrosPrimalIcon />,
  identity: "polkadot",
  value: "cybros-primal",
  chain: "cybros",
  symbol: "CBT",
  decimals: 12,
  color: "#3765DC",
  colorSecondary: "rgba(55, 101, 220, 0.1)",
  buttonColor: "#000000",
  modules: {
    identity: false,
  },
  nodes: [
    { name: "Cybros", url: "wss://node-rpc.cybros.network" },
  ],
  useOnChainBlockData: false,
};

const chains = {
  "cybros-primal": cybrosPrimal
};

export default chains;
