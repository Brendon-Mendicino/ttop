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
  ChartArea,
  Filler,
} from "chart.js";
import { useEffect, useRef, useState } from "react";
import { Line } from "react-chartjs-2";
import { listen } from "@tauri-apps/api/event";
import { CpuStat } from "../lib/bindings/CpuStat";
import { SingleCpu } from "../lib/bindings/SingleCpu";
import { range } from "../lib/array/array";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
);

const options: ChartOptions<"line"> = {
  responsive: true,
  plugins: {
    legend: {
      display: false,
    },
  },
  scales: {
    x: {
      display: false,
    },
    y: {
      min: 0,
      max: 100,
    },
  },
};

function createGradient(
  ctx: CanvasRenderingContext2D,
  area: ChartArea,
  alpha: number,
) {
  const gradient = ctx.createLinearGradient(0, area.bottom, 0, area.top);

  gradient.addColorStop(0, `rgba(0, 255, 0, ${alpha})`);
  gradient.addColorStop(0.5, `rgba(255, 255, 0, ${alpha})`);
  gradient.addColorStop(1, `rgba(255, 0, 0, ${alpha})`);

  return gradient;
}

export function Cpu() {
  const [dataset, setDataset] = useState([...range(0, 20)].map(() => 0));
  const [dd, setDd] = useState<SingleCpu>();
  const chartRef = useRef<ChartJS<"line"> | null>(null);

  const data = {
    labels: [...range(0, 20)],
    datasets: [
      {
        data: dataset,
        fill: true,
        borderColor:
          (chartRef.current &&
            createGradient(
              chartRef.current.ctx,
              chartRef.current.chartArea,
              1.0,
            )) ??
          "red",
        backgroundColor:
          (chartRef.current &&
            createGradient(
              chartRef.current.ctx,
              chartRef.current.chartArea,
              0.2,
            )) ??
          "red",
      },
    ],
  };

  useEffect(() => {
    const unlisten = listen<CpuStat>("cpu", (event) => {
      setDd(event.payload.cpu);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    setDataset((d) => {
      if (dd) {
        d = d.slice(1);
        d.push(dd.user);
      }
      return d;
    });
  }, [dd]);

  return <Line data={data} options={options} ref={chartRef} />;
}
