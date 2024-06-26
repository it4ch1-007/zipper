import React, {useEffect, useState} from 'react';
import { useLocation } from 'react-router-dom';
import { createStyles} from '@mantine/core';
import { ScrollArea } from '@mantine/core';
import { Button } from '@mantine/core';
import {Modal,TextInput} from '@mantine/core';
import { invoke } from '@tauri-apps/api';
import {Alert} from '@mantine/core';


function Zip_extr() {
    const location = useLocation();
    const {zipName} = location.state || {};
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [inputValue, setInputValue] = useState('');
    // const openModal = () => setIsModalOpen(false);
    const closeModal = () => setshowPrompt(true);
    const [responseMetadata,setresponseMetadata] = useState('');
    const [responseTree,setresponseTree] = useState('');
    const [fileNames, setFileNames] = useState([]);
    const [showAlert,setShowAlert] = useState(false);
    const [prompt,setshowPrompt] = useState(true);
    const [variable,setVariable] = useState(false);

    const handleSubmission = (event) => {
      closeModal();
  };

  const handlePasswordChange = (event) => {
        
    setInputValue(event.currentTarget.value);
  };
    const handlereadzipfiles = async() => {
      if(prompt){
        invoke('read_zip_files', { zippath: zipName })
      .then(response => {
        setFileNames(response);
      })
      .catch(error => {
        invoke('error_printer');

      }
    
    ,[]);
      }
      else{
      invoke('read_zip_files_pswd', { zippath: zipName, pswd:inputValue })
      .then(response => {
        setFileNames(response);
      })
      .catch(error => {
        invoke('error_printer_pswd');

      }
    
    ,[]);
      }
    }
    const handleExtraction = async() =>{
        console.log("extract clicked");
        // await invoke('extract_zip',{zippath:zipName});
        await invoke('config_write',{zipPath:zipName});
        setShowAlert(true);

        setTimeout(() => {
          setShowAlert(false);
        },3000);
                
    };
   
    // const handlefilestree = async() => {
    //     try{
    //         console.log("tree fn called");
    //         const responseTree = await invoke('read_zip_files', { zippath: zipName});
    //         setresponseTree(responseTree);
    //     }
    //     catch{}
    // }
    
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
      if(!variable){
        setVariable(true);
        setshowPrompt(response);
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
        console.log(prompt);
          fetchData();
          handleMetadata();
          handlePriorCheck();
          handlereadzipfiles();

      }, [zipName,prompt]);
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
            {showAlert && (
              <div style={{ position: 'fixed', bottom: '20px', right: '20px', zIndex: 1000 }}>
        <Alert variant="light" color="blue" title="Alert title">
          Zip extracted...
        </Alert>
        </div>
      )}
            <ScrollArea w={300} h={200} scrollbars="x" type='hover'>
        {<p>{responseMetadata}</p>}
    </ScrollArea>       
    <ScrollArea style={{ height: 300, width: 700 }}>
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