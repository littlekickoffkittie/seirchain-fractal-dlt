import React, { useState } from 'react';

function CreateWallet() {
  const [userId, setUserId] = useState('');
  const [walletAddress, setWalletAddress] = useState('');

  const handleCreateWallet = async () => {
    if (!userId) return;
    try {
      const response = await fetch('http://127.0.0.1:8080/api/create_wallet', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ user_id: userId }),
      });
      if (!response.ok) throw new Error('Failed to create wallet');
      const data = await response.json();
      setWalletAddress(data.address);
      alert(`Wallet created with address: ${data.address}`);
    } catch (error) {
      alert('Error creating wallet: ' + error.message);
    }
  };

  return (
    <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
      <h3 className="text-xl font-semibold mb-4 high-contrast-text">Create Wallet</h3>
      <div className="flex flex-col space-y-4">
        <input
          type="text"
          placeholder="Enter User ID"
          value={userId}
          onChange={(e) => setUserId(e.target.value)}
          className="p-2 rounded-md bg-gray-200 dark:bg-gray-700"
        />
        <button
          onClick={handleCreateWallet}
          className="bg-black dark:bg-white text-white dark:text-black font-semibold py-2 px-4 rounded-md hover:bg-gray-800 dark:hover:bg-gray-200 transition"
        >
          Create Wallet
        </button>
        {walletAddress && (
          <p className="text-sm text-gray-400 mt-2">
            New Wallet Address: {walletAddress}
          </p>
        )}
      </div>
    </div>
  );
}

export default CreateWallet;
