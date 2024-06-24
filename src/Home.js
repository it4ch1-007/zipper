import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Button, Group } from '@mantine/core';
import { open } from '@tauri-apps/api/dialog';
import { createStyles} from '@mantine/styles';
import {NavLink,useNavigate} from 'react-router-dom';

function Home() {
    const navigate = useNavigate();
  const [lines, setLines] = useState([]);
  const [zipName, setZipname] = useState('');

  useEffect(() => {
    async function fetchLines() {
      try {
        const response = await invoke('config_read');
        setLines(response);
      } catch (err) {
        console.error(err);
      }
    }
    fetchLines();
  }, []);

  const handleopendialog = async () => {
    console.log("button clicked");
    try {
      const selectedPath = await open();
      console.log("Selected Path", selectedPath);
      setZipname(selectedPath);
      console.log(zipName);
      navigate('/Zip_extr',{state: {zipName:selectedPath}});
    } catch (err) {
      console.log(err);
    }
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

  const { classes } = useStyles();

  return (
    <div>
      <h1>Home Page</h1>
      <p>Welcome to the home page!</p>
      <Group position="center">
        <Button onClick={handleopendialog}>Open</Button>
      </Group>
      <Group direction="vertical" align="center" style={{ marginTop: '20px' }}>
        {lines.map((line, index) => (
          <Button key={index} component={NavLink} className={classes.navLink} fullWidth>
            {line}
          </Button>
        ))}
      </Group>
    </div>
  );
}

export default Home;
