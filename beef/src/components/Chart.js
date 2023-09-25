import { useEffect, useState, useRef } from "react";
import { createChart, ColorType } from 'lightweight-charts';
import data from './data';
import { useWrappedConn } from "../lib/useConnection";
import { useParams } from 'react-router-dom';
const Chart = () => {

    const chartContainerRef = useRef();
    const conn = useWrappedConn();
    const [orderBook, setOrderBook] = useState(null);
    let { id } = useParams();
 
    useEffect(()=> {
        const number = parseInt(id);

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
    }, [id, conn.query])
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