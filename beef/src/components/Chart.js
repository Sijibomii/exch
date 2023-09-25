import { useEffect, useState, useRef } from "react";
import { createChart, ColorType } from 'lightweight-charts';
import data from './data';
import { useWrappedConn } from "../lib/useConnection";
import { useLocation } from "react-router-dom"

const Chart = () => {

    const chartContainerRef = useRef();
    const conn = useWrappedConn();
    const [orderBook, setOrderBook] = useState(null);

    const location = useLocation();

    function extractIdFromPath(path) {
        // Define a regular expression to match the "/trade/" followed by digits
        const regex = /\/trade\/(\d+)/;
        
        // Use the exec method to search for a match in the path
        const match = regex.exec(path);
        
        // Check if there is a match and return the captured ID or null
        if (match && match[1]) {
          return match[1]; // The ID is captured in the first capturing group (match[1])
        } else {
          return null; // Return null if no match is found
        }
      }
 
    useEffect(()=> {
        const number = parseInt(extractIdFromPath(location.pathname));

     conn.query.getOrderBook(number)
        .then((response) => {
        // Handle the successful response here
            console.log('OrderBook response:', response);
        })
        .catch((error) => {
            // Handle errors here
            console.error('OrderBook error:', error);
        });

        // send 
    }, [conn.query])
    useEffect(() => {
        const width = window.innerWidth;
        const chartProperties = {
            width: width,
            height:500,
            timeScale:{
              timeVisible:true,
              secondsVisible:false,
            },
            layout: {
                background: { type: ColorType.Solid, color: '#1D1E2F' },
                textColor: 'white',
            },
            grid: {
                vertLines:{
                    visible: false
                },
                horzLines:{
                    visible: false
                }
            }
            
        }
        const handleResize = () => {
            chart.applyOptions({ width: chartContainerRef.current.clientWidth });
        };

        const chartData = data.map(d => {
            return {time:d[0]/1000,open:parseFloat(d[1]),high:parseFloat(d[2]),low:parseFloat(d[3]),close:parseFloat(d[4])}
        });

        const chart = createChart(chartContainerRef.current, chartProperties);
        chart.timeScale().fitContent();

        const newSeries = chart.addCandlestickSeries();
        newSeries.setData(chartData);

        window.addEventListener('resize', handleResize);

        return () => {
            window.removeEventListener('resize', handleResize);

            chart.remove();
        };
    },[]);
    return (
       <div className="" ref={chartContainerRef}>

       </div>
    );
}

export default Chart;