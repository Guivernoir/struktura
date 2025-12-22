import { useState, useRef, useEffect, useMemo } from "react";
import PropTypes from "prop-types";
import Icon from "../Icon";

const DateTimePicker = ({
  label,
  name,
  value,
  onChange,
  required = false,
  disabled = false,
  error,
  helpText,
  stepMinutes = 15, // Interval for the time wheel
  daysRange = 30, // Number of days to show in the wheel
}) => {
  const [open, setOpen] = useState(false);
  const rootRef = useRef(null);

  // Helper: Format ISO to Date objects or specific strings
  const dateValue = value ? new Date(value) : new Date();

  // Generate Date Options (Next X days)
  const dateOptions = useMemo(() => {
    const dates = [];
    const start = new Date();
    for (let i = -daysRange; i <= daysRange; i++) {
      const d = new Date(start);
      d.setDate(start.getDate() + i);
      dates.push(d);
    }
    return dates;
  }, [daysRange]);

  // Generate Time Options based on stepMinutes
  const timeOptions = useMemo(() => {
    const times = [];
    for (let h = 0; h < 24; h++) {
      for (let m = 0; m < 60; m += stepMinutes) {
        times.push(
          `${String(h).padStart(2, "0")}:${String(m).padStart(2, "0")}`
        );
      }
    }
    return times;
  }, [stepMinutes]);

  // Handle outside clicks to close popover
  useEffect(() => {
    if (!open) return;
    const handleClick = (e) => {
      if (rootRef.current && !rootRef.current.contains(e.target))
        setOpen(false);
    };
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, [open]);

  const handleSelect = (date, timeStr) => {
    const [hours, minutes] = timeStr.split(":");
    const newDate = new Date(date);
    newDate.setHours(parseInt(hours), parseInt(minutes), 0, 0);
    onChange({ target: { name, value: newDate.toISOString() } });
  };

  const formatDisplay = (val) => {
    if (!val) return "Select Date & Time";
    return new Intl.DateTimeFormat(undefined, {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    }).format(new Date(val));
  };

  return (
    <div className="relative w-full" ref={rootRef}>
      <label className="text-sm font-medium text-charcoal-700 dark:text-steel-300 mb-2 block">
        {label} {required && <span className="text-red-500">*</span>}
      </label>

      <button
        type="button"
        disabled={disabled}
        onClick={() => setOpen(!open)}
        className={`w-full flex items-center justify-between px-4 py-3 border rounded-xl bg-white dark:bg-charcoal-800 transition-all ${
          error ? "border-red-500" : "border-sand-300 dark:border-charcoal-700"
        } ${
          disabled ? "opacity-50 cursor-not-allowed" : "hover:border-indigo-400"
        }`}
      >
        <span className="text-sm">{formatDisplay(value)}</span>
        <Icon name="Calendar" size={18} />
      </button>

      {open && (
        <div className="absolute z-50 mt-2 w-full bg-white dark:bg-charcoal-900 border border-sand-200 dark:border-charcoal-700 shadow-2xl rounded-2xl overflow-hidden animate-in fade-in zoom-in duration-150">
          <div className="flex h-64 relative">
            {/* Selection Overlay (The "Highlight" bar) */}
            <div className="absolute top-1/2 left-0 w-full h-10 -translate-y-1/2 pointer-events-none border-y border-indigo-500/20 bg-indigo-50/10 dark:bg-indigo-500/5" />

            {/* Date Wheel */}
            <Wheel
              options={dateOptions}
              value={dateOptions.find(
                (d) => d.toDateString() === dateValue.toDateString()
              )}
              format={(d) =>
                d.toLocaleDateString(undefined, {
                  weekday: "short",
                  month: "short",
                  day: "numeric",
                })
              }
              onSelect={(d) =>
                handleSelect(
                  d,
                  `${dateValue.getHours()}:${dateValue.getMinutes()}`
                )
              }
            />

            {/* Time Wheel */}
            <Wheel
              options={timeOptions}
              value={`${String(dateValue.getHours()).padStart(2, "0")}:${String(
                dateValue.getMinutes()
              ).padStart(2, "0")}`}
              format={(t) => t}
              onSelect={(t) => handleSelect(dateValue, t)}
            />
          </div>

          <div className="p-3 border-t border-sand-100 dark:border-charcoal-800 flex justify-end gap-2 bg-sand-50/50 dark:bg-charcoal-800/50">
            <button
              onClick={() => setOpen(false)}
              className="px-4 py-2 text-sm font-semibold text-indigo-600 dark:text-indigo-400 hover:bg-indigo-50 dark:hover:bg-indigo-900/30 rounded-lg transition"
            >
              Done
            </button>
          </div>
        </div>
      )}

      {error && <p className="text-xs text-red-500 mt-1">{error}</p>}
      {helpText && !error && (
        <p className="text-xs text-charcoal-500 mt-1">{helpText}</p>
      )}
    </div>
  );
};

/**
 * Internal Scroll Wheel Component
 */
const Wheel = ({ options, value, onSelect, format }) => {
  const scrollRef = useRef(null);

  // Scroll to current value on mount
  useEffect(() => {
    const index = options.indexOf(value);
    if (index !== -1 && scrollRef.current) {
      scrollRef.current.scrollTop = index * 40;
    }
  }, []);

  const handleScroll = () => {
    if (!scrollRef.current) return;
    const index = Math.round(scrollRef.current.scrollTop / 40);
    if (options[index] && options[index] !== value) {
      onSelect(options[index]);
    }
  };

  return (
    <div
      ref={scrollRef}
      onScroll={handleScroll}
      className="flex-1 overflow-y-scroll snap-y snap-mandatory no-scrollbar py-24"
      style={{ scrollbarWidth: "none" }}
    >
      {options.map((opt, i) => (
        <div
          key={i}
          className={`h-10 flex items-center justify-center snap-center transition-opacity ${
            opt === value
              ? "opacity-100 font-bold text-indigo-600 dark:text-indigo-400"
              : "opacity-40 text-sm"
          }`}
        >
          {format(opt)}
        </div>
      ))}
    </div>
  );
};

DateTimePicker.propTypes = {
  label: PropTypes.string.isRequired,
  name: PropTypes.string.isRequired,
  value: PropTypes.string,
  onChange: PropTypes.func.isRequired,
  stepMinutes: PropTypes.number,
  daysRange: PropTypes.number,
  required: PropTypes.bool,
  disabled: PropTypes.bool,
  error: PropTypes.string,
  helpText: PropTypes.string,
};

export default DateTimePicker;
