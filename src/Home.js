import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Button, Group, ScrollArea } from '@mantine/core';
import { open } from '@tauri-apps/api/dialog';
import { createStyles} from '@mantine/styles';
import {useNavigate} from 'react-router-dom';
import {Text, Paper } from '@mantine/core';

function Home() {
  const navigate = useNavigate(); //To navigate to other webpages using states or functions
  const [lines, setLines] = useState([]); //To store the lines inside the configuration file having the zip names recently visited.
  const [zipName, setZipname] = useState(''); //Store the zipname

  //Calling the functions to read the file whenever the page is loaded.
  useEffect(() => {
    async function fetchLines() {
      try {
        const response = await invoke('config_read');
        setLines(response);
      } catch (err) {
      }
    }
    fetchLines();
  }, []);

  //To open the file dialog box and navigate to Zip_extr.js
  const handleopendialog = async () => {
    try {
      const selectedPath = await open();
      setZipname(selectedPath);
      navigate('/Zip_extr',{state: {zipName:selectedPath}}); //navigate to Zip_extraction page with the zipname passed as a prop
    } catch (err) {
    }
  };
  
  //custom styles to highlight the ScrollArea
  const highlightedStyles = createStyles((theme) => ({
    highlightedText: {
      backgroundColor: theme.colors.blue[100], // Customize the highlight background color
      padding: '8px',
      display:'block',
      width: '100%',
      borderRadius: '4px',
      marginBottom: '8px', // Adjust spacing between lines
      textAlign: 'center',
    },
  }));

  return (
    <div>
      <h1>Home</h1>
      <Group position="center">
        <Button onClick={handleopendialog}>Open</Button>
        <br></br>
        <br></br>
        <br></br>
      </Group>
      <p align='center'>Recent Files Extracted:</p>
      <ScrollArea w={300} h={250}>
      <Paper style={{ padding: '20px', maxWidth: '400px', margin: 'auto', height: '250px' }}>
        {/* Mapping the zip file names inside the config.txt */}
      <Group direction="vertical" align="center" style={{ marginTop: '20px',width: '100%' }}>
        {lines.map((line, index) => (
          <Text key={index} className={highlightedStyles.highlightedText}>
            {line}
          </Text>
        ))}
      </Group>
    </Paper>
    </ScrollArea>
      
    </div>
  );
}

export default Home;
