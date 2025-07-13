import React, { useState } from 'react';

function CheckBalance() {
  const [address, setAddress] = useState('');
  const [balance, setBalance] = useState(null);

  const handleCheckBalance = async () => {
    if (!address) return;
    try {
      const response = await fetch('http://127.0.0.1:8080/api/sign_in', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ address }),
      });
      if (!response.ok) throw new Error('Wallet not found');
      const data = await response.json();
      setBalance(data.balance);
    } catch (error) {
      alert('Error checking balance: ' + error.message);
      setBalance(null);
    }
  };

  return (
    <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
      <h3 className="text-xl font-semibold mb-4 high-contrast-text">Check Balance</h3>
      <div className="flex flex-col space-y-4">
        <input
          type="text"
          placeholder="Enter Wallet Address"
          value={address}
          onChange={(e) => setAddress(e.target.value)}
          className="p-2 rounded-md bg-gray-200 dark:bg-gray-700"
        />
        <button
          onClick={handleCheckBalance}
          className="bg-black dark:bg-white text-white dark:text-black font-semibold py-2 px-4 rounded-md hover:bg-gray-800 dark:hover:bg-gray-200 transition"
        >
          Check Balance
        </button>
        {balance !== null && (
          <p className="text-sm text-gray-400 mt-2">
            Balance: {balance} WAC
          </p>
        )}
      </div>
    </div>
  );
}

export default CheckBalance;
