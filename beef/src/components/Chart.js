import { useEffect, useState, useRef } from "react";
import { createChart, ColorType } from 'lightweight-charts';

import { useWrappedConn } from "../lib/useConnection";
import { useLocation } from "react-router-dom"
import { useOrderBookStore } from "../lib/useOrderBook";

const Chart = () => {

    const chartContainerRef = useRef();
    const conn = useWrappedConn();
    const [orderBook, setOrderBook] = useState([]);
    const [render, setRender] = useState(false)
    const { orders } = useOrderBookStore();

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
    //   const orders = useOrderBookStore.getState().orders;

    
 
    useEffect(()=> {
        const number = parseInt(extractIdFromPath(location.pathname));

        async function getOrders(number) { 
            await conn.query.getOrderBook(number);
        }
        
        getOrders(number);

    }, [location.pathname])

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

    

        const chart = createChart(chartContainerRef.current, chartProperties);
        chart.timeScale().fitContent();

        const newSeries = chart.addCandlestickSeries();
        const ord = orders.filter((order) => ((order !== undefined) && (order !== null)) &&  (order.close !== null) && (order.high !== null ));
        newSeries.setData(ord);

        const tradeHandler = (event) => {
            newSeries.update(event.detail)
            console.log('Custom trade event received', event.detail);
        }

        window.addEventListener('trade', tradeHandler);
        window.addEventListener('resize', handleResize);

        

        return () => {
            window.removeEventListener('resize', handleResize);
            window.removeEventListener('trade', tradeHandler);
            chart.remove();
        };
    },[orders]);
    return (
       <div className="" ref={chartContainerRef}>

       </div>
    );
}

export default Chart;