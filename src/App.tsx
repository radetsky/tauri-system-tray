import reactLogo from "./assets/react.svg";
import { open } from '@tauri-apps/api/shell';

import "./App.css";

function App() {

  async function open_process(openWith: string = '') {
    if (openWith === '') {
      await open('https://github.com/tauri-apps/tauri');
    } else {
      await open('https://github.com/tauri-apps/tauri', openWith);
    }
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <div className="row">
        <button onClick={() => {open_process('firefox')}}>Firefox</button>
        <button onClick={() => {open_process('chrome')}}>Chrome</button>
        <button onClick={() => {open_process('')}}>Default</button>

      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>
      <p>Клацніть на кнопках Firefox, Chrome та Default, щоб відкрити посилання на GitHub в цих трьох браузерах відповидно.</p>

    </div>
  );
}

export default App;
