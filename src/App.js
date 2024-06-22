import logo from './logo.svg';
import React,{useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import './App.css';


function App() {
  const [result,setResult] = useState('');
  const handlePrint = async () => {
    try {
      const response = await invoke('extract_zip',{path: "C:\\Users\\akshi\\Downloads\\filers.zip"});
      setResult(response);
      // alert('Zip file extracted successfully!');
    } catch (error) {
      setResult("error fetching the zip file!!");
      // console.error('Failed to extract zip file:', error);
      // alert('Failed to extract zip file');
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={handlePrint}>Print</button>
        <p> Result: {result}</p>
      </header>
    </div>
  );
}

export default App;
