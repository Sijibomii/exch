const { Worker, isMainThread, parentPort, workerData } = require('worker_threads');
const os = require("os");

const emails = [
  "johndoe123@example.com", 
  "sarah.smith@email.net", 
  "mike.jones@hotmail.com", 
  "lisa.johnson@gmail.com", 
  "robert.wilson@yahoo.com", 
  "emily.brown@example.org",
  "chris.martin@email.com",
  "amanda.white@hotmail.net",
  "david.clark@gmail.org",
  "ashley.williams@example.net",
  "jennifer.davis@email.com",
  "matthew.miller@yahoo.net",
  "laura.jackson@hotmail.com",
  "william.jones@email.net",
  "olivia.davis@example.com"
];

function retPriceIncr(inputNumber) {
  if (inputNumber === 0 || inputNumber === 1) {
    return 1;
  } else if (inputNumber === 2 || inputNumber === 3) {
    return 2;
  } else if (inputNumber === 4 || inputNumber === 5) {
    return 3;
  } else if (inputNumber === 6 || inputNumber === 7) {
    return 4;
  } else if (inputNumber === 8 || inputNumber === 9) {
    return 5;
  } else {
    return null; // Handle invalid input, if needed
  }
}

function start() {
  if (isMainThread){
    for (let i = 0; i < Object.keys(os.cpus()).length; i++) {
      const worker = new Worker('./src/worker.js', { workerData: { 
        email: emails[i], 
        password: emails[i], 
        ticker_id: 0, 
        isBuyer: i % 2 === 0, 
        priceIncrement: retPriceIncr(i)
      }});
      console.log("stareted worker: ", i)
      worker.on('message', (message) => {
        console.log(`Message from worker: ${message}`);
        worker.terminate();
      });
    
      worker.postMessage('Start worker');
    }
  }
  
}
start();
