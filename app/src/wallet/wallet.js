const connectWithProvider = async (wallet, displayElement) => {
  try {
    const accounts = await wallet.provider.request({
      method: "eth_requestAccounts",
    });
    if (accounts.length > 0) {
      displayElement.innerHTML = `Connected address: ${accounts[0]}`;
      return { provider: wallet.provider, accounts };
    } else {
      displayElement.innerHTML = "No accounts found.";
    }
  } catch (error) {
    console.error("Error with connecting", error);
    displayElement.innerHTML = "Error with connecting to the wallet.";
  }
};

export function listProviders() {
  const element = document.querySelector("#providerButtons");
  const displayElement = document.querySelector("#walletAddress");
  window.addEventListener("eip6963:announceProvider", (event) => {
    const button = document.createElement("button");

    button.innerHTML = `
        <img src="${event.detail.info.icon}" alt="${event.detail.info.name}" />
        <div>${event.detail.info.name}</div>
      `;

    button.onclick = async () => {
      const connection = await connectWithProvider(
        event.detail,
        displayElement,
      );
      if (connection) {
        // Save the provider and accounts globally or pass them to your functions as needed
        window.currentProvider = connection.provider;
        window.currentAccounts = connection.accounts;
      }
    };
    element.appendChild(button);
  });

  window.dispatchEvent(new Event("eip6963:requestProvider"));
}
