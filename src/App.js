
import React,{useState,useEffect} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import './App.css';
import {AppShell, Navbar, Text, MediaQuery, Burger, ActionIcon, Group,MantineProvider,Button} from '@mantine/core';
import {Header} from '@mantine/core';
import { SunIcon,MoonIcon } from '@modulz/radix-icons';
import {createStyles,useMantineTheme} from '@mantine/styles'
import {MemoryRouter, NavLink, Route,Routes,useNavigate} from 'react-router-dom';
import Zip_extr from './Zip_extr';
import Home from './Home';
import Settings from './Settings';
import About from './About';
import Exit from './Exit';


function App() {

  const [zipName,setZipname] = useState('');//to store the state of zip name chosen by the user.
  const [lines,setLines] = useState('');
  const views = [{
    path: '/',
    name: 'Home',
    exact: true,
    component: Home
  },
    {
      path: 'about',
      name: 'About',
      exact: true,
      component: About
    },{
      path: 'Zip_extr',
      name: 'Zip extraction',
      exact: true,
      component: Zip_extr
    },{
      path: 'exit',
      name: 'Exit',
      exact: true,
      component: Exit
    }
    ]
    

  
  const [opened,setOpened] = useState(false); //Storing the state of the links 
  const defaultColorScheme = 'dark';

  const [colorScheme,setColorScheme] = useState(defaultColorScheme);//setting the colorscheme

 //Function to toggle the theme of the webpages
  const toggleColorScheme = value => {
    const newValue = value || (colorScheme === 'dark' ? 'light': 'dark');
    setColorScheme(newValue);
  };


  //adding some custom styles
  const useStyles = createStyles((theme) => ({
    navLink: {
      display: 'block',
      width: '100%', 
      padding: theme.spacing.xs,
      borderRadius: theme.radius.md,
      color: colorScheme === 'dark' ? theme.colors.dark[0]: theme.black,
      textDecoration: 'none',

      '&:hover': {
        backgroundColor: colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[1],
      }
    },
    navLinkActive: {
      backgroundColor: colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[1],
    }
  }));

  //calling the function to read the config.txt file as soon as the homepage appears.
  const { classes } = useStyles();
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

  return (
    <MantineProvider theme={{colorScheme: colorScheme, fontFamily: 'Open Sans, sans serif'}} withGlobalStyles >
      <MemoryRouter>
        <AppShell padding = "md" navbarOffsetBreakpoint = "sm" fixed
        navbar = {
          <Navbar width = {{sm:200}} padding="xs" hidden={!opened} hiddenBreakpoint="sm">
          {
            views.map((view,index) =>  
              <NavLink align="left" to={view.path} key={index} onClick = {() => setOpened(false)} className = {({ isActive }) => classes.navLink + ' ' + (isActive ? classes.navLinkActive: '')}>
                <Group>
                  <Text>{view.name}
                  </Text>
                </Group>
              </NavLink>
            )}
          </Navbar>
        }
        header = {
          <Header height = {120} padding = "sm">
            <div style = {{ display: 'flex',alignItems: 'center', height: '100%' }}>
              <MediaQuery
               largerThan = "sm" styles = {{display: 'none'}}>
                <Burger
                  opened = {opened}
                  onClick = {() => setOpened((o) => !o)}
                  size = "sm"
                  color = {useMantineTheme().colors.gray[6]}
                  mr= "xl"
                  />
               </MediaQuery>
               <Text tt="uppercase" ta="center"> Zipper </Text>
               <div style={{marginLeft: "auto"}}>
               {/* Adding the icon to toggle the theme */}
                <ActionIcon variant="default" onClick={() => toggleColorScheme()} size={30}>
                  {colorScheme === 'dark' ? <SunIcon/> : <MoonIcon/>}
                </ActionIcon>
               </div>
            </div>
          </Header>
        }
        styles={theme => ({
          main: {backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[0] },
        })}>
          
      {/* this is to give the routes . they are the only ones inside the AppShell as content everything else is just attributes or tags */}
        <Routes>{
          views.map((view,index) => <Route key = {index} exact = {view.exact} path={view.path} element = {<view.component/>}/>)
}       </Routes>
        
        </AppShell>
      </MemoryRouter>
    </MantineProvider>
  );
}

export default App;
