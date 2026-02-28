async function connectWallet() {
  const walletText = document.getElementById("wallet");

  try {
    // check Freighter properly
    if (!window.freighterApi) {
      walletText.innerText = "❌ Freighter extension not found";
      return;
    }

    const isAllowed = await window.freighterApi.isAllowed();

    if (!isAllowed) {
      await window.freighterApi.setAllowed();
    }

    const publicKey = await window.freighterApi.getPublicKey();
    userAddress = publicKey;

    walletText.innerText = "✅ Connected: " + publicKey;

  } catch (e) {
    console.error(e);
    walletText.innerText = "❌ Wallet error: " + e.message;
  }
}