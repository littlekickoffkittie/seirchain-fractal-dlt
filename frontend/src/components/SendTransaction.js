import React, { useState } from 'react';

function SendTransaction() {
  const [fromAddress, setFromAddress] = useState('');
  const [toAddress, setToAddress] = useState('');
  const [amount, setAmount] = useState('');

  const handleSendTransaction = async () => {
    if (!fromAddress || !toAddress || !amount) return;
    try {
      const response = await fetch('http://127.0.0.1:8080/api/send_transaction', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          from_address: fromAddress,
          to_address: toAddress,
          amount: parseFloat(amount),
        }),
      });
      if (!response.ok) throw new Error('Failed to send transaction');
      alert('Transaction sent successfully!');
    } catch (error) {
      alert('Error sending transaction: ' + error.message);
    }
  };

  return (
    <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
      <h3 className="text-xl font-semibold mb-4 high-contrast-text">Send Transaction</h3>
      <div className="flex flex-col space-y-4">
        <input
          type="text"
          placeholder="From Address"
          value={fromAddress}
          onChange={(e) => setFromAddress(e.target.value)}
          className="p-2 rounded-md bg-gray-200 dark:bg-gray-700"
        />
        <input
          type="text"
          placeholder="To Address"
          value={toAddress}
          onChange={(e) => setToAddress(e.target.value)}
          className="p-2 rounded-md bg-gray-200 dark:bg-gray-700"
        />
        <input
          type="number"
          placeholder="Amount"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          className="p-2 rounded-md bg-gray-200 dark:bg-gray-700"
        />
        <button
          onClick={handleSendTransaction}
          className="bg-black dark:bg-white text-white dark:text-black font-semibold py-2 px-4 rounded-md hover:bg-gray-800 dark:hover:bg-gray-200 transition"
        >
          Send Transaction
        </button>
      </div>
    </div>
  );
}

export default SendTransaction;
