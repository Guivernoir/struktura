import PropTypes from "prop-types";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Cell,
} from "recharts";

const ChartsView = ({ results, theme }) => {
  // 1. Filter only numeric results for charting
  const chartData = results
    .filter((r) => !isNaN(parseFloat(r.value)))
    .map((r) => ({
      name: r.label,
      value: parseFloat(r.value),
      unit: r.unit,
      fullLabel: r.label, // Store full label for tooltip
    }));

  if (chartData.length === 0) return null;

  // Colors based on Struktura theme - UPDATED to use Indigo accent for Engineer
  const barColor = theme === "dark" ? "#818cf8" : "#4f46e5"; // indigo-400 : indigo-600
  const gridColor = theme === "dark" ? "#374151" : "#e5e7eb"; // charcoal-700 : sand-200
  const textColor = theme === "dark" ? "#9ca3af" : "#4b5563"; // steel-400 : charcoal-600

  return (
    <div className="h-[300px] w-full mt-6">
      <h4 className="text-sm font-semibold text-charcoal-500 dark:text-steel-400 mb-4 uppercase tracking-wider">
        Data Analysis
      </h4>
      <ResponsiveContainer width="100%" height="100%">
        <BarChart
          data={chartData}
          margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
          layout="vertical" // Horizontal bars are often better for long labels
        >
          <CartesianGrid
            strokeDasharray="3 3"
            stroke={gridColor}
            horizontal={false}
          />
          <XAxis type="number" stroke={textColor} fontSize={12} />
          <YAxis
            dataKey="name"
            type="category"
            width={100}
            stroke={textColor}
            fontSize={12}
            tickFormatter={(val) =>
              val.length > 12 ? `${val.substring(0, 12)}...` : val
            }
          />
          <Tooltip
            cursor={{ fill: theme === "dark" ? "#1f2937" : "#f3f4f6" }}
            contentStyle={{
              backgroundColor: theme === "dark" ? "#111827" : "#ffffff",
              borderColor: gridColor,
              color: theme === "dark" ? "#ffffff" : "#000000",
            }}
            formatter={(value, name, props) => [
              `${value} ${props.payload.unit || ""}`,
              props.payload.fullLabel,
            ]}
          />
          <Bar dataKey="value" radius={[0, 4, 4, 0]} barSize={32}>
            {chartData.map((entry, index) => (
              <Cell key={`cell-${index}`} fill={barColor} />
            ))}
          </Bar>
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
};

ChartsView.propTypes = {
  results: PropTypes.array.isRequired,
  theme: PropTypes.string.isRequired,
};

export default ChartsView;
