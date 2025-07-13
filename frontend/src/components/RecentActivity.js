import React, { useState, useEffect } from 'react';

function RecentActivity() {
  const [transactions, setTransactions] = useState([]);

  useEffect(() => {
    const fetchRecentActivity = async () => {
      try {
        const response = await fetch('http://127.0.0.1:8080/api/recent_activity');
        if (!response.ok) throw new Error('Failed to fetch recent activity');
        const data = await response.json();
        setTransactions(data.transactions);
      } catch (error) {
        console.error('Error fetching recent activity:', error);
      }
    };

    fetchRecentActivity();
    const interval = setInterval(fetchRecentActivity, 5000); // Poll every 5 seconds

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
      <h3 className="text-xl font-semibold mb-4 high-contrast-text">Recent Activity</h3>
      <div className="flex flex-col space-y-4">
        {transactions.length > 0 ? (
          transactions.map((tx, index) => (
            <div key={index} className="p-2 rounded-md bg-gray-200 dark:bg-gray-700">
              <p>From: {tx.from_address}</p>
              <p>To: {tx.to_address}</p>
              <p>Amount: {tx.amount} WAC</p>
            </div>
          ))
        ) : (
          <p>No recent activity.</p>
        )}
      </div>
    </div>
  );
}

export default RecentActivity;
