import React, { useEffect, useRef, useState } from 'react';
import './App.css';
import CreateWallet from './components/CreateWallet';
import CheckBalance from './components/CheckBalance';
import SendTransaction from './components/SendTransaction';
import RecentActivity from './components/RecentActivity';

function App() {
  const [darkMode, setDarkMode] = useState(true);
  const canvasRef = useRef(null);

  useEffect(() => {
    const html = document.documentElement;
    if (darkMode) {
      html.classList.add('dark');
    } else {
      html.classList.remove('dark');
    }
  }, [darkMode]);

  useEffect(() => {
    // All the script from the original index.html is now here
    // Triad Matrix Explorer Visualization
    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');

    function resizeCanvas() {
      const rect = canvas.getBoundingClientRect();
      canvas.width = rect.width * window.devicePixelRatio;
      canvas.height = rect.height * window.devicePixelRatio;
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    }
    resizeCanvas();
    window.addEventListener('resize', () => {
      resizeCanvas();
      drawTriad(currentDepth);
    });

    let currentDepth = 0;
    const maxDepth = 6;
    const minDepth = 0;

    let miningData = {};

    function generateMiningData(depth) {
      const count = Math.pow(3, depth);
      miningData[depth] = [];
      for (let i = 0; i < count; i++) {
        miningData[depth][i] = Math.random();
      }
    }

    for (let d = 0; d <= maxDepth; d++) {
      generateMiningData(d);
    }

    function drawFilledTriangle(p1, p2, p3, fillStyle) {
      ctx.beginPath();
      ctx.moveTo(p1.x, p1.y);
      ctx.lineTo(p2.x, p2.y);
      ctx.lineTo(p3.x, p3.y);
      ctx.closePath();
      ctx.fillStyle = fillStyle;
      ctx.fill();
      ctx.strokeStyle = '#444';
      ctx.lineWidth = 1;
      ctx.stroke();
    }

    function drawSierpinski(p1, p2, p3, depth, index) {
      if (depth === 0) {
        const activity = miningData[currentDepth][index] || 0;
        const greenIntensity = Math.floor(50 + activity * 205);
        const color = `rgba(0, ${greenIntensity}, 0, 0.8)`;
        drawFilledTriangle(p1, p2, p3, color);
        return;
      }
      const mid12 = { x: (p1.x + p2.x) / 2, y: (p1.y + p2.y) / 2 };
      const mid23 = { x: (p2.x + p3.x) / 2, y: (p2.y + p3.y) / 2 };
      const mid31 = { x: (p3.x + p1.x) / 2, y: (p3.y + p1.y) / 2 };

      drawSierpinski(p1, mid12, mid31, depth - 1, index * 3);
      drawSierpinski(mid12, p2, mid23, depth - 1, index * 3 + 1);
      drawSierpinski(mid31, mid23, p3, depth - 1, index * 3 + 2);
    }

    function drawTriad(depth) {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      const width = canvas.clientWidth;
      const height = canvas.clientHeight;
      const size = Math.min(width, height) * 0.9;
      const heightTriangle = (size * Math.sqrt(3)) / 2;
      const offsetX = (width - size) / 2;
      const offsetY = (height - heightTriangle) / 2;
      const p1 = { x: offsetX, y: offsetY + heightTriangle };
      const p2 = { x: offsetX + size, y: offsetY + heightTriangle };
      const p3 = { x: offsetX + size / 2, y: offsetY };

      if (depth === 0) {
        drawFilledTriangle(p1, p2, p3, 'rgba(255, 255, 255, 0.9)');
        ctx.strokeStyle = '#888';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(p1.x, p1.y);
        ctx.lineTo(p2.x, p2.y);
        ctx.lineTo(p3.x, p3.y);
        ctx.closePath();
        ctx.stroke();
      } else {
        drawSierpinski(p1, p2, p3, depth, 0);
      }
    }

    drawTriad(currentDepth);
    updateInfo();

    function updateInfo() {
      document.getElementById('triadDepthDisplay').textContent = currentDepth;
      const totalTx = 1000 * Math.pow(3, currentDepth);
      document.getElementById('totalTransactionsDisplay').textContent = totalTx.toLocaleString();
    }

    document.getElementById('increaseDepthBtn').addEventListener('click', () => {
      if (currentDepth < maxDepth) {
        currentDepth++;
        drawTriad(currentDepth);
        updateInfo();
      }
    });

    document.getElementById('decreaseDepthBtn').addEventListener('click', () => {
      if (currentDepth > minDepth) {
        currentDepth--;
        drawTriad(currentDepth);
        updateInfo();
      }
    });

  }, []);

  return (
    <div className={`bg-gray-100 text-gray-900 transition-colors duration-300 ${darkMode ? 'dark dark:bg-gray-900 dark:text-gray-200' : ''}`}>
      <div className="flex h-screen">
        <aside className="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col">
          <div className="p-6 text-center border-b border-gray-200 dark:border-gray-700">
            <img src="/logo.png" alt="SeirChain Logo" className="mx-auto mb-4" style={{ width: '100px' }} />
            <h1 className="text-2xl font-bold high-contrast-text">
              SEIRCHAIN
            </h1>
            <p className="text-xs text-gray-500 dark:text-gray-400">
              Fractal Ledger Dashboard
            </p>
          </div>
          <nav className="flex-1 p-4 space-y-2 overflow-y-auto">
            <a className="flex items-center px-4 py-2 text-gray-700 dark:text-gray-300 bg-gray-200 dark:bg-gray-700 rounded-md" href="#staking">
              <svg className="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 10v-1m0 0c-1.657 0-3-.895-3-2s1.343-2 3-2 3 .895 3 2-1.343 2-3 2z" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2">
                </path>
              </svg>
              Staking
            </a>
            <a className="flex items-center px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-md" href="#pof">
              <svg className="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path d="M13 10V3L4 14h7v7l9-11h-7z" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2">
                </path>
              </svg>
              PoF Mining
            </a>
            <a className="flex items-center px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-md" href="#explorer">
              <svg className="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2">
                </path>
                <path d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2">
                </path>
              </svg>
              Explorer
            </a>
            <a className="flex items-center px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-md" href="#network">
              <svg className="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path d="M8 9l4-4 4 4m0 6l-4 4-4-4" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2">
                </path>
              </svg>
              Network
            </a>
            <a className="flex items-center px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-md" href="#developer">
              <svg className="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2">
                </path>
              </svg>
              Developer
            </a>
          </nav>
          <div className="p-4 border-t border-gray-200 dark:border-gray-700">
            <div className="flex items-center justify-between">
              <span className="text-sm">
                Dark Mode
              </span>
              <button aria-label="Toggle dark mode" className="relative inline-flex items-center h-6 rounded-full w-11 transition-colors bg-gray-300 dark:bg-gray-600" onClick={() => setDarkMode(!darkMode)}>
                <span className="sr-only">
                  Enable dark mode
                </span>
                <span className={`inline-block w-4 h-4 transform bg-white rounded-full transition-transform ${darkMode ? 'translate-x-6' : 'translate-x-1'}`}>
                </span>
              </button>
            </div>
          </div>
        </aside>
        <main className="flex-1 p-6 lg:p-10 overflow-y-auto">
          <header className="flex justify-between items-center mb-8">
            <h2 className="text-3xl font-bold high-contrast-text">
              Dashboard
            </h2>
          </header>
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-10 mb-10">
            <CreateWallet />
            <CheckBalance />
            <SendTransaction />
          </div>
          <RecentActivity />
          <section className="mb-10" id="staking">
            <h3 className="text-xl font-semibold mb-4 high-contrast-text">
              Staking Portfolio
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
              <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
                <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">
                  Total Staked
                </h4>
                <p className="text-3xl font-bold high-contrast-text mt-2">
                  -- WAC
                </p>
                <p className="text-sm text-gray-400 mt-1 italic">
                  Not initialized
                </p>
              </div>
              <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
                <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">
                  Rewards Earned
                </h4>
                <p className="text-3xl font-bold high-contrast-text mt-2">
                  -- WAC
                </p>
                <p className="text-sm text-gray-400 mt-1 italic">
                  Not initialized
                </p>
              </div>
              <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
                <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">
                  Active Stakes
                </h4>
                <p className="text-3xl font-bold high-contrast-text mt-2">
                  --
                </p>
                <p className="text-sm text-gray-400 mt-1 italic">
                  Not initialized
                </p>
              </div>
              <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700 flex flex-col justify-center">
                <button className="w-full bg-black dark:bg-white text-white dark:text-black font-semibold py-2 px-4 rounded-md cursor-not-allowed opacity-50" disabled="" type="button">
                  Manage Stakes
                </button>
                <button className="w-full mt-2 text-sm text-gray-400 cursor-not-allowed italic" disabled="" type="button">
                  Rebalance
                </button>
              </div>
            </div>
            <div className="mt-6 bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
              <h4 className="text-lg font-semibold mb-4 high-contrast-text">
                Staking Growth
              </h4>
              <div className="chart-container flex items-center justify-center h-64 text-gray-400 italic">
                Data not available
              </div>
            </div>
          </section>
          <section className="mb-10" id="pof">
            <h3 className="text-xl font-semibold mb-4 high-contrast-text">
              Proof-of-Fractal (PoF) Performance
            </h3>
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <div className="lg:col-span-1 space-y-6">
                <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
                  <div className="flex justify-between items-center">
                    <h4 className="text-lg font-semibold high-contrast-text">
                      PoF Mining Status
                    </h4>
                    <span className="text-sm font-medium text-gray-400 flex items-center italic">
                      <span className="w-2 h-2 bg-gray-400 rounded-full mr-2">
                      </span>
                      Not Active
                    </span>
                  </div>
                  <div className="mt-4 flex space-x-2">
                    <button className="w-full bg-gray-400 text-gray-700 font-semibold py-2 px-4 rounded-md cursor-not-allowed opacity-50" disabled="" type="button">
                      Stop
                    </button>
                    <button className="w-full bg-gray-300 dark:bg-gray-700 text-gray-700 dark:text-gray-400 font-semibold py-2 px-4 rounded-md cursor-not-allowed opacity-50" disabled="" type="button">
                      Settings
                    </button>
                  </div>
                </div>
                <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
                  <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">
                    Puzzle Rate
                  </h4>
                  <p className="text-3xl font-bold high-contrast-text mt-2">
                    -- Sol/s
                  </p>
                </div>
                <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
                  <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">
                    Validated / Invalid Triads
                  </h4>
                  <p className="text-3xl font-bold high-contrast-text mt-2">
                    <span className="text-gray-400">
                      --
                    </span>
                    /
                    <span className="text-gray-400">
                      --
                    </span>
                  </p>
                </div>
              </div>
              <div className="lg:col-span-2 bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700 flex items-center justify-center h-64 text-gray-400 italic">
                Data not available
              </div>
            </div>
          </section>
          <section className="mb-10" id="explorer">
            <h3 className="text-xl font-semibold mb-4 high-contrast-text">
              Triad Matrix Explorer
            </h3>
            <div className="bg-black p-4 rounded-lg border border-gray-700 flex flex-col">
              <canvas aria-label="3D interactive visualization of Genesis Triad fractal breaking down into smaller fractal triangles representing mining operations" id="triad-canvas" ref={canvasRef} role="img" tabIndex="0">
              </canvas>
              <div className="flex justify-between items-center mt-4 text-white p-2">
                <div>
                  <p className="text-sm">
                    Triad Depth:
                    <span className="font-mono" id="triadDepthDisplay">
                      0
                    </span>
                  </p>
                  <p className="text-sm">
                    Total Transactions:
                    <span className="font-mono" id="totalTransactionsDisplay">
                      0
                    </span>
                  </p>
                </div>
                <div className="flex space-x-2">
                  <button aria-label="Increase triad depth" className="bg-gray-800 p-2 rounded-md hover:bg-gray-700" id="increaseDepthBtn" type="button" title="Increase Triad Depth">
                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                      <path clipRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-11a1 1 0 10-2 0v2H7a1 1 0 100 2h2v2a1 1 0 102 0v-2h2a1 1 0 100-2h-2V7z" fillRule="evenodd">
                      </path>
                    </svg>
                  </button>
                  <button aria-label="Decrease triad depth" className="bg-gray-800 p-2 rounded-md hover:bg-gray-700" id="decreaseDepthBtn" type="button" title="Decrease Triad Depth">
                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                      <path clipRule="evenodd" d="M5 10a1 1 0 011-1h8a1 1 0 110 2H6a1 1 0 01-1-1z" fillRule="evenodd">
                      </path>
                    </svg>
                  </button>
                </div>
              </div>
            </div>
          </section>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-10">
            <section id="network">
              <h3 className="text-xl font-semibold mb-4 high-contrast-text">
                Network &amp; Consensus
              </h3>
              <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700 space-y-4">
                <div className="flex justify-between items-center">
                  <span className="font-medium">
                    Network Health
                  </span>
                  <span className="text-gray-400 font-semibold italic">
                    --
                  </span>
                </div>
                <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2.5">
                  <div className="bg-gray-400 h-2.5 rounded-full" style={{ width: '0%' }}>
                  </div>
                </div>
                <div className="flex justify-between items-center">
                  <span className="font-medium">
                    Node Sync
                  </span>
                  <span className="text-gray-400 font-semibold italic">
                    --
                  </span>
                </div>
                <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2.5">
                  <div className="bg-gray-400 h-2.5 rounded-full" style={{ width: '0%' }}>
                  </div>
                </div>
                <div className="flex justify-between items-center">
                  <span className="font-medium">
                    Consensus
                  </span>
                  <span className="text-gray-400 italic">
                    --
                  </span>
                </div>
                <div className="text-sm text-gray-400 border-t border-gray-200 dark:border-gray-700 pt-4 italic">
                  <p>
                    Last Consensus Event: --
                  </p>
                  <p>
                    Peers Connected: --
                  </p>
                </div>
              </div>
            </section>
            <section id="developer">
              <h3 className="text-xl font-semibold mb-4 high-contrast-text">
                Developer &amp; API
              </h3>
              <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700 space-y-4">
                <div className="flex justify-between items-center">
                  <span className="font-medium">
                    SVM Status
                  </span>
                  <span className="text-gray-400 font-semibold italic">
                    --
                  </span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="font-medium">
                    API Gateway
                  </span>
                  <span className="text-gray-400 font-semibold italic">
                    --
                  </span>
                </div>
                <div className="border-t border-gray-200 dark:border-gray-700 pt-4 flex space-x-2">
                  <button className="w-full bg-gray-400 text-gray-700 font-semibold py-2 px-4 rounded-md cursor-not-allowed opacity-50" disabled="" type="button">
                    SDKs
                  </button>
                  <button className="w-full bg-gray-400 text-gray-700 font-semibold py-2 px-4 rounded-md cursor-not-allowed opacity-50" disabled="" type="button">
                    Docs
                  </button>
                </div>
              </div>
            </section>
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;
