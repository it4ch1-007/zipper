import logo from './logo.svg';
import React,{useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import './App.css';
import {AppShell, Navbar, Text, MediaQuery, Burger, ActionIcon, Group,MantineProvider} from '@mantine/core';
import {Header} from '@mantine/core';
import { SunIcon,MoonIcon } from '@modulz/radix-icons';
import {createStyles,useMantineTheme} from '@mantine/styles'
import {MemoryRouter, NavLink, Route,Routes} from 'react-router-dom';
import Home from './Home';
import Settings from './Settings';

function App() {

  const views = [{
    path: '/',
    name: 'Home',
    exact: true,
    component: Home
  },{path: 'settings',
    name: 'Settings',
    exact: true,
    component: Settings}]
  const [result,setResult] = useState('');

  //nav bar
  const [opened,setOpened] = useState(false);
  const defaultColorScheme = 'dark';
  // console.log(defaultColorScheme);
  const [colorScheme,setColorScheme] = useState(defaultColorScheme);

  //fn for changing the colorScheme
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
      backgroundColor: colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[1],
    }
  }));

  const { classes } = useStyles();

  return (
    <MantineProvider theme={{colorScheme: colorScheme, fontFamily: 'Open Sans, sans serif'}} withGlobalStyles >
      <MemoryRouter>
        <AppShell padding = "md" navbarOffsetBreakpoint = "sm" fixed
        navbar = {
          <Navbar width = {{sm:200}} padding="xs" hidden={!opened} hiddenBreakpoint="sm">
          {
            //this maps the given views array according to the given conditions
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
               <Text>WELCOME....</Text>
               <div style={{marginLeft: "auto"}}>
                <ActionIcon variant="default" onClick={() => toggleColorScheme()} size={30}>
                  {colorScheme === 'dark' ? <SunIcon/> : <MoonIcon/>}
                </ActionIcon>
               </div>
            </div>
          </Header>
        }
        styles={theme => ({
          main: {backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[0] },

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




  // const handlePrint = async () => {
  //   try {
  //     const response = await invoke('extract_zip',{path: "C:\\Users\\akshi\\Downloads\\filers.zip"});
  //     setResult(response);
  //     // alert('Zip file extracted successfully!');
  //   } catch (error) {
  //     setResult("error fetching the zip file!!");
  //     // console.error('Failed to extract zip file:', error);
  //     // alert('Failed to extract zip file');
  //   }
  // };

//   return (
//     <div className="App">
//       <header className="App-header">
//         <button onClick={handlePrint}>Print</button>
//         <p> Result: {result}</p>
//       </header>
//     </div>
//   );
// }

export default App;
