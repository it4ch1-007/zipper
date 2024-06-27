import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Button, Group } from '@mantine/core';
import { open } from '@tauri-apps/api/dialog';
import { createStyles} from '@mantine/styles';
import {NavLink,useNavigate} from 'react-router-dom';
import {Modal,TextInput} from '@mantine/core';
import { Table, TableCell, TableRow,Text, Paper } from '@mantine/core';

function Home() {
  const navigate = useNavigate();
  const [lines, setLines] = useState([]);
  const [zipName, setZipname] = useState('');
  
  // const openModal = () => setIsModalOpen(true);


 
  
  
  useEffect(() => {
    async function fetchLines() {
      try {
        const response = await invoke('config_read');
        setLines(response);
      } catch (err) {
        // console.error(err);
      }
    }
    fetchLines();
  }, []);

  const handleopendialog = async () => {
    // console.log("button clicked");
    try {
      const selectedPath = await open();
      // console.log("Selected Path", selectedPath);
      setZipname(selectedPath);
      // handlePriorCheck();
      // console.log(zipName);
      navigate('/Zip_extr',{state: {zipName:selectedPath}});
    } catch (err) {
      // console.log(err);
    }
  };

  

  const handledirectnavigation = async(zipPath) => {
    // console.log("hello");
    setZipname(zipPath);
    navigate('/Zip_extr',{state: {zipName:zipPath}});
    // console.log(zipPath);
  };
  const useStyles = createStyles((theme) => ({
    navLink: {
      display: 'block',
      width: '100%',
      padding: theme.spacing.xs,
      borderRadius: theme.radius.md,
      color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.black,
      textDecoration: 'none',
      '&:hover': {
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[1],
      },
    },
  }));
  const barStyles = createStyles((theme) => ({
    bar: {
      height: '4px',
      backgroundColor: theme.colors.blue[500], // Customize the bar color here
      marginBottom: '10px', // Adjust spacing between bars
    },
  }));
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
  const { classes } = useStyles();

  return (
    <div>
      <h1>Home Page</h1>
      <p>Welcome to the home page!</p>
      <Group position="center">
        <Button onClick={handleopendialog}>Open</Button>
      </Group>
      <Paper style={{ padding: '20px', maxWidth: '400px', margin: 'auto' }}>
      <Group direction="vertical" align="center" style={{ marginTop: '20px',width: '100%' }}>
        {lines.map((line, index) => (
          <Text key={index} className={highlightedStyles.highlightedText}>
            {line}
          </Text>
        ))}
      </Group>
    </Paper>
      
    </div>
  );
}

export default Home;
