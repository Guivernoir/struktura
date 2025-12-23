import React, { useState, useRef, useEffect, useMemo } from "react";
import PropTypes from "prop-types";
import Icon from "../Icon";

/**
 * An improved, grid-based DateTimePicker.
 * Replaces the wheel logic with a clear calendar and time-slot selector.
 */
const DateTimePicker = ({
  label,
  name,
  value,
  onChange,
  required = false,
  disabled = false,
  error,
  helpText,
  stepMinutes = 15,
}) => {
  const [open, setOpen] = useState(false);
  const rootRef = useRef(null);

  // Normalize current value to a Date object
  const dateValue = useMemo(
    () => (value ? new Date(value) : new Date()),
    [value]
  );

  // Handle outside clicks
  useEffect(() => {
    if (!open) return;
    const handleClick = (e) => {
      if (rootRef.current && !rootRef.current.contains(e.target))
        setOpen(false);
    };
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, [open]);

  // Format display text for the trigger button
  const formatDisplay = (val) => {
    if (!val) return "Select Date & Time";
    return new Intl.DateTimeFormat(undefined, {
      month: "short",
      day: "numeric",
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    }).format(new Date(val));
  };

  const updateDateTime = (newDate) => {
    onChange({ target: { name, value: newDate.toISOString() } });
  };

  const handleDateChange = (e) => {
    const selected = new Date(e.target.value);
    const updated = new Date(dateValue);
    updated.setFullYear(
      selected.getFullYear(),
      selected.getMonth(),
      selected.getDate()
    );
    updateDateTime(updated);
  };

  const handleTimeChange = (timeStr) => {
    const [hours, minutes] = timeStr.split(":");
    const updated = new Date(dateValue);
    updated.setHours(parseInt(hours), parseInt(minutes), 0, 0);
    updateDateTime(updated);
  };

  // Generate time slots based on stepMinutes
  const timeSlots = useMemo(() => {
    const slots = [];
    for (let h = 0; h < 24; h++) {
      for (let m = 0; m < 60; m += stepMinutes) {
        slots.push(
          `${String(h).padStart(2, "0")}:${String(m).padStart(2, "0")}`
        );
      }
    }
    return slots;
  }, [stepMinutes]);

  return (
    <div className="relative w-full" ref={rootRef}>
      <label className="text-sm font-medium text-charcoal-700 dark:text-steel-300 mb-1.5 block">
        {label} {required && <span className="text-red-500">*</span>}
      </label>

      <button
        type="button"
        disabled={disabled}
        onClick={() => setOpen(!open)}
        className={`w-full flex items-center justify-between px-4 py-3 border rounded-xl bg-white dark:bg-charcoal-800 transition-all shadow-sm ${
          error ? "border-red-500" : "border-sand-300 dark:border-charcoal-700"
        } ${
          disabled
            ? "opacity-50 cursor-not-allowed"
            : "hover:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20"
        }`}
      >
        <span className="text-sm text-charcoal-900 dark:text-white">
          {formatDisplay(value)}
        </span>
        <Icon name="Calendar" size={18} className="text-charcoal-400" />
      </button>

      {open && (
        <div className="absolute z-50 mt-2 w-72 sm:w-80 bg-white dark:bg-charcoal-900 border border-sand-200 dark:border-charcoal-700 shadow-2xl rounded-2xl overflow-hidden animate-in fade-in slide-in-from-top-2 duration-200">
          <div className="p-4 space-y-4">
            {/* Native Date Selection for ease of use */}
            <div>
              <span className="text-xs font-semibold uppercase tracking-wider text-charcoal-500 dark:text-steel-500 block mb-2">
                Date
              </span>
              <input
                type="date"
                className="w-full p-2 bg-sand-50 dark:bg-charcoal-800 border border-sand-200 dark:border-charcoal-700 rounded-lg text-sm"
                value={dateValue.toISOString().split("T")[0]}
                onChange={handleDateChange}
              />
            </div>

            {/* Time Slot Grid */}
            <div>
              <span className="text-xs font-semibold uppercase tracking-wider text-charcoal-500 dark:text-steel-500 block mb-2">
                Time
              </span>
              <div className="grid grid-cols-3 gap-2 max-h-40 overflow-y-auto pr-1 custom-scrollbar">
                {timeSlots.map((time) => {
                  const isActive =
                    `${String(dateValue.getHours()).padStart(2, "0")}:${String(
                      dateValue.getMinutes()
                    ).padStart(2, "0")}` === time;

                  return (
                    <button
                      key={time}
                      type="button"
                      onClick={() => handleTimeChange(time)}
                      className={`py-1.5 text-xs rounded-md border transition-colors ${
                        isActive
                          ? "bg-indigo-600 border-indigo-600 text-white font-bold"
                          : "bg-white dark:bg-charcoal-800 border-sand-200 dark:border-charcoal-700 text-charcoal-700 dark:text-steel-300 hover:border-indigo-400"
                      }`}
                    >
                      {time}
                    </button>
                  );
                })}
              </div>
            </div>
          </div>

          <div className="p-3 border-t border-sand-100 dark:border-charcoal-800 bg-sand-50/50 dark:bg-charcoal-800/50 flex justify-end">
            <button
              type="button"
              onClick={() => setOpen(false)}
              className="px-4 py-2 text-sm font-bold text-indigo-600 dark:text-indigo-400 hover:bg-indigo-50 dark:hover:bg-indigo-900/30 rounded-lg transition"
            >
              Done
            </button>
          </div>
        </div>
      )}

      {error ? (
        <p className="text-xs text-red-500 mt-1">{error}</p>
      ) : helpText ? (
        <p className="text-xs text-charcoal-500 mt-1">{helpText}</p>
      ) : null}
    </div>
  );
};

DateTimePicker.propTypes = {
  label: PropTypes.string.isRequired,
  name: PropTypes.string.isRequired,
  value: PropTypes.string,
  onChange: PropTypes.func.isRequired,
  required: PropTypes.bool,
  disabled: PropTypes.bool,
  error: PropTypes.string,
  helpText: PropTypes.string,
  stepMinutes: PropTypes.number,
};

export default DateTimePicker;
