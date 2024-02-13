import { event } from '@tauri-apps/api';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  ChartOptions,
} from 'chart.js';
import { useEffect, useRef, useState } from 'react';
import { Line } from 'react-chartjs-2';
import { Proc } from '../lib/bindings/Proc';
import { listen } from '@tauri-apps/api/event';
import { CpuStat } from '../lib/bindings/CpuStat';
import { SingleCpu } from '../lib/bindings/SingleCpu';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

const options: ChartOptions<"line"> = {
  responsive: true,
  plugins: {
    legend: {
      position: 'top' as const,
    },
    title: {
      display: true,
      text: 'Chart.js Line Chart',
    },
  },
  scales: {
    x: {
      ticks: {
        display: false, // Disable x axis values display
      }
    },
    y: {
      min: 0,
      max: 100,
    }
  },
  
};

export function Cpu() {
  const [labels, setLabels] = useState([1, 2, 3, 4, 5, 6, 7]);
  const [dataset, setDataset] = useState([1, 2, 3, 4, 5, 6, 7]);
  const [dd, setDd] = useState<SingleCpu>();

  const data = {
    labels,
    datasets: [
      {
        label: 'Dataset 1',
        data: dataset,
        borderColor: 'rgb(255, 99, 132)',
        backgroundColor: 'rgba(255, 99, 132, 0.5)',
      },
    ],
  };

  useEffect(() => {
    const unlisten = listen<CpuStat>("cpu", (event) => {
      setDd(event.payload.cpu)
    });

    return () => {
      unlisten.then(f => f());
    }
  }, []);

    const [n, setN] = useState(0);

  useEffect(() => {
    setDataset(d => {
      if (dd) {
        d = d.slice(1);
        d.push(dd.user);
      }
      return d
    });
  }, [dd]);


  return <>
    <button onClick={() => setN(n => n + 1)}>Click me {n}</button>
    <Line data={data} options={options} />
  </>
}
