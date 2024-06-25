import React, {useEffect, useState} from 'react';
import { useLocation } from 'react-router-dom';
import { createStyles} from '@mantine/core';
import { ScrollArea } from '@mantine/core';
import { Button } from '@mantine/core';
import {Modal,TextInput} from '@mantine/core';
import { Box } from '@mantine/core';
import { invoke } from '@tauri-apps/api';
// import { Notification  } from '@mantine/core';

function Zip_extr() {
    const location = useLocation();
    const {zipName} = location.state || {};
    // const [opened, { open, close }] = useDisclosure(false);
    const [inputValue, setInputValue] = useState('hello default');
    const [isModalOpen, setIsModalOpen] = useState(false);
    const openModal = () => setIsModalOpen(true);
    const closeModal = () => setIsModalOpen(false);
    const [responseMetadata,setresponseMetadata] = useState('');
    const [responseTree,setresponseTree] = useState('');
    const [fileNames, setFileNames] = useState([]);
    // const [notificationPermission, setNotificationPermission] = useState('default');

    
    // const requestNotificationPermission = () => {
    //     if ('Notification' in window) {
    //       Notification.requestPermission().then((permission) => {
    //         setNotificationPermission(permission);
    //       });
    //     }
    //   };
    //   const showNotification = () => {
    //     if (notificationPermission === 'granted') {
    //       new Notification('Function Execution Complete', {
    //         body: 'Your function has finished executing.',
    //         icon: 'path/to/icon.png', // Optional icon for the notification
    //       });
    //     } else if (notificationPermission === 'default') {
    //       requestNotificationPermission();
    //     }
    //   };

    const handlereadzipfiles = async() => {
        invoke('read_zip_files', { zippath: zipName })
      .then(response => {
        setFileNames(response);
      })
      .catch(error => {
        console.error('Error fetching file names:', error);
      },[]);

    }
    const handleExtraction = async() =>{
        console.log("extract clicked");
        invoke('extract_zip',{zippath:zipName});
        // notifications.show({
        //     title: 'Extracting..',
        //     message: 'Zip is extracted...',
        //   });
        
                
    };
    const handlePasswordChange = (event) => {
        // console.log("called")
        // console.log(event.currentTarget.value);
        setInputValue(event.currentTarget.value);
      };
    const handlefilestree = async() => {
        try{
            console.log("tree fn called");
            const responseTree = await invoke('read_zip_files', { zippath: zipName});
            setresponseTree(responseTree);
        }
        catch{}
    }
    const handleSubmission = (event) => {
        // console.log("password entered: ",inputValue);
        // setInputValue(event.target.value);
        closeModal();
    };
    const handleMetadata = async () => {
        try{
        // console.log("hello");
        const responseMetadata = await invoke('read_metadata',{archive: zipName});
        setresponseMetadata(responseMetadata);
        }
        catch(err){
            console.error("Failed to fetch metadata");
        }

    }
    useEffect(() => {
        const fetchData = async () => {
          try {
            console.log("Fetching data from the zip:", zipName);
           
          } catch (err) {
            console.log("Error fetching the zip name", err);
          }
        };
          fetchData();
          handleMetadata();
          handlereadzipfiles();

      }, [zipName]);
    const useStyles = createStyles((theme) => ({
        scrollArea: {
          maxHeight: 50,
          maxWidth: 50,
          overflow: 'auto',
          border: `1px solid ${theme.colors.gray[4]}`, // Optional: Add a border for better visibility
        },
      }));


    return (
        
        <div>
            <h1>
                Zip:</h1> {zipName}<br></br>
            
            <Button onClick={handleExtraction}>Extract zip</Button>
            <Button onClick={openModal}>Password test</Button>
            <ScrollArea w={300} h={200} scrollbars="x" type='hover'>
      <Box w={600}>
        {<p>{responseMetadata}</p>}
      </Box>
    </ScrollArea>       
    <ScrollArea style={{ height: 300, width: 400 }}>
      {fileNames.map((fileName, index) => (
        <div key={index}>{fileName}</div>
      ))}
    </ScrollArea>     
      <Modal opened={isModalOpen} onClose={closeModal} title="Password Test">
        <TextInput
          label="Password"
          placeholder="Enter password"
          onChange={handlePasswordChange}
          
        />
        <Button onClick={handleSubmission}>Submit</Button>
      </Modal>
      {/* <p>Zip Name: {zipName}<br/></p> */}
      
    </div>
    );
    
}

export default Zip_extr;