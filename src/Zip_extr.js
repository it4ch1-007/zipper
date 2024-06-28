import React, {useEffect, useState} from 'react';
import { useLocation } from 'react-router-dom';
import { createStyles} from '@mantine/core';
import { ScrollArea } from '@mantine/core';
import { Button } from '@mantine/core';
import {Modal,TextInput} from '@mantine/core';
import { invoke } from '@tauri-apps/api';
import {Alert} from '@mantine/core';


function Zip_extr() {
    let flag=false;
    const location = useLocation();
    const {zipName} = location.state || {};
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [inputValue, setInputValue] = useState('trllo');
    const closeModal = () => setshowPrompt(true);
    const [fileNames, setFileNames] = useState([]);
    const [showAlert,setShowAlert] = useState(false);
    const [prompt,setshowPrompt] = useState();
    const [responseMetadata,setresponseMetadata] = useState('');
    

    const handleSubmission = (event) => {
      closeModal();
      invoke('read_zip_files_pswd', { zippath: zipName, pswd: inputValue })
      .then(response => {
        console.log("flag value: ",flag);
        setFileNames(response);
        flag=true;
      })
      .catch(error => {
        invoke('error_printer_pswd');
      });
  };

  const handlePasswordChange = (event) => {
        
    setInputValue(event.currentTarget.value);
  };
    const handlereadzipfiles = async() => {
      
      if(!flag){
      flag=true;
      console.log("inside read zip files handler: ",prompt);
      if(prompt){
        invoke('read_zip_files', { zippath: zipName ,pswd:inputValue})
      .then(response => {
        setFileNames(response);
      })
      .catch(error => {
        invoke('error_printer');

      }
    
    ,[]);
      }
    }
    }


    const handleExtraction = async() =>{
        if(prompt){
        await invoke('extract_zip',{zippath:zipName,pswd:inputValue});
        }
        else{
          await invoke('extract_zip_pswd',{zipPath:zipName,pswd:inputValue});
        }
        await invoke('config_write',{zipPath:zipName});
        setShowAlert(true);

        setTimeout(() => {
          setShowAlert(false);
        },3000);
                
    };
   
    
    const handleMetadata = async () => {
        try{
        const responseMetadata = await invoke('read_metadata',{archive: zipName});
        setresponseMetadata(responseMetadata);
        
        }
        catch(err){
            console.error("Failed to fetch metadata");
        }

    }
    const handlePriorCheck = async() => {
      
      const response = await invoke('prior_check',{zippath:zipName});
        setshowPrompt(response);
        console.log("Response given : ",response);

      
  }
    useEffect(() => {
          handleMetadata();
          handlePriorCheck();
      }, [zipName]);
      useEffect(() => {
        console.log("value of prompt inside useEffect: ",prompt);
        if (prompt) {
          handlereadzipfiles();
        }
      }, [zipName,prompt]);    
      const useStyles = createStyles((theme) => ({
        scrollArea: {
          overflow: 'auto',
          border: `1px solid ${theme.colors.dark[7]}`, // Optional: Add a border for better visibility
          backgroundColor: theme.colors.dark[5],
        },
      }));
      const {classes} = useStyles();


    return (
        
        <div>
            <h1>
                Zip:</h1> {zipName}<br></br>
            
            <Button onClick={handleExtraction}>Extract zip</Button>
            {showAlert && (
              <div style={{ position: 'fixed', bottom: '20px', right: '20px', zIndex: 1000 }}>
        <Alert variant="light" color="blue" title="Alert title">
          Zip {zipName} extracted in the same directory...
        </Alert>
        </div>
      )}
      <p>Zip metadata: </p>
            <ScrollArea w={300} h={200} scrollbars="x" type='hover' className={classes.scrollArea}>
        {responseMetadata}
    </ScrollArea>       
    <br></br>
    <br></br>
      <p>Zip Files tree: </p>
    <ScrollArea style={{ height: 300, width: 700 }} className={classes.scrollArea}>
      {fileNames.map((fileName, index) => (
        <div key={index}>{fileName}</div>
      ))}
    </ScrollArea>  
    <Modal opened={!prompt} onClose={closeModal} title="Password Test">
        <TextInput
          label="Password"
          placeholder="Enter password"
          onChange={handlePasswordChange}
        />
        <Button onClick={handleSubmission}>Submit</Button>
      </Modal>   
    </div>
    );
    
}

export default Zip_extr;