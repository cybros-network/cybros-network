const chains = Object.freeze({
  "cybros-primal": "cybros-primal",
});

const ss58Format = Object.freeze({
  [chains["cybros-primal"]]: 42,
});

function getSs58Format(chain) {
  return ss58Format[chain];
}

const assetsModuleChains = [];

const uniquesModuleChains = [];

module.exports = {
  chains,
  assetsModuleChains,
  uniquesModuleChains,
  getSs58Format,
};
