import { useState } from "react";
import PropTypes from "prop-types";
import Icon from "../Icon";
import {
  isoToDatetimeLocal,
  datetimeLocalToISO,
} from "../../hooks/engineer/types";

/**
 * ArrayInput - Dynamic array field with add/remove functionality
 * For managing arrays of objects or primitives in engineering parameters
 *
 * Well, that was quite the strategic decision, wasn't it?
 * Arrays that actually behave like arrays.
 */
const ArrayInput = ({
  label,
  name,
  value = [],
  onChange,
  itemSchema,
  required = false,
  helpText,
  t,
}) => {
  const [expandedItems, setExpandedItems] = useState(new Set([0]));

  const toggleItem = (index) => {
    setExpandedItems((prev) => {
      const next = new Set(prev);
      if (next.has(index)) {
        next.delete(index);
      } else {
        next.add(index);
      }
      return next;
    });
  };

  const addItem = () => {
    const newItem = createEmptyItem(itemSchema);
    const newArray = [...value, newItem];
    onChange({
      target: {
        name,
        value: newArray,
      },
    });
    // Auto-expand the new item
    setExpandedItems((prev) => new Set([...prev, newArray.length - 1]));
  };

  const removeItem = (index) => {
    const newArray = value.filter((_, i) => i !== index);
    onChange({
      target: {
        name,
        value: newArray,
      },
    });
    // Clean up expanded items
    setExpandedItems((prev) => {
      const next = new Set();
      prev.forEach((i) => {
        if (i < index) next.add(i);
        else if (i > index) next.add(i - 1);
      });
      return next;
    });
  };

  const updateItem = (index, field, fieldValue) => {
    const newArray = [...value];
    newArray[index] = {
      ...newArray[index],
      [field]: fieldValue,
    };
    onChange({
      target: {
        name,
        value: newArray,
      },
    });
  };

  return (
    <div className="space-y-3">
      {/* Header */}
      <div className="flex items-center justify-between">
        <label className="text-sm font-medium text-charcoal-700 dark:text-steel-300">
          {label}
          {required && <span className="text-red-500 ml-1">*</span>}
        </label>
        <button
          type="button"
          onClick={addItem}
          className="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-indigo-600 dark:text-indigo-400 hover:bg-indigo-50 dark:hover:bg-indigo-900/20 rounded-lg transition"
        >
          <Icon name="Plus" size={14} />
          {t?.engineer?.form?.add_item || "Add Item"}
        </button>
      </div>

      {helpText && (
        <p className="text-xs text-charcoal-500 dark:text-steel-500">
          {helpText}
        </p>
      )}

      {/* Array Items */}
      <div className="space-y-2">
        {value.length === 0 ? (
          <div className="p-4 border-2 border-dashed border-sand-300 dark:border-charcoal-700 rounded-xl text-center text-sm text-charcoal-500 dark:text-steel-500">
            {t?.engineer?.form?.no_items ||
              "No items yet. Click 'Add Item' to create one."}
          </div>
        ) : (
          value.map((item, index) => (
            <div
              key={index}
              className="border border-sand-300 dark:border-charcoal-700 rounded-xl overflow-hidden"
            >
              {/* Item Header */}
              <div className="flex items-center justify-between px-4 py-2 bg-sand-50 dark:bg-charcoal-800/50">
                <button
                  type="button"
                  onClick={() => toggleItem(index)}
                  className="flex items-center gap-2 text-sm font-medium text-charcoal-700 dark:text-steel-300 hover:text-charcoal-900 dark:hover:text-white transition"
                >
                  <Icon
                    name={
                      expandedItems.has(index) ? "ChevronDown" : "ChevronRight"
                    }
                    size={16}
                  />
                  {t?.engineer?.form?.item || "Item"} #{index + 1}
                </button>
                <button
                  type="button"
                  onClick={() => removeItem(index)}
                  className="flex items-center gap-1 px-2 py-1 text-xs font-medium text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 rounded transition"
                >
                  <Icon name="Trash2" size={14} />
                  {t?.engineer?.form?.remove || "Remove"}
                </button>
              </div>

              {/* Item Fields */}
              {expandedItems.has(index) && (
                <div className="p-4 space-y-3">
                  {itemSchema.fields.map((field) => (
                    <ArrayItemField
                      key={field.name}
                      field={field}
                      value={item[field.name] || ""}
                      onChange={(fieldValue) =>
                        updateItem(index, field.name, fieldValue)
                      }
                    />
                  ))}
                </div>
              )}
            </div>
          ))
        )}
      </div>
    </div>
  );
};

/**
 * Individual field within an array item
 */
const ArrayItemField = ({ field, value, onChange }) => {
  const handleChange = (e) => {
    let newValue = e.target.value;

    // Type conversion
    if (field.type === "number") {
      const num = parseFloat(newValue);
      newValue = isNaN(num) ? newValue : num;
    } else if (field.type === "integer") {
      const num = parseInt(newValue, 10);
      newValue = isNaN(num) ? newValue : num;
    } else if (field.type === "datetime") {
      // Convert datetime-local to ISO 8601
      newValue = datetimeLocalToISO(newValue);
    }

    onChange(newValue);
  };

  // For display: Convert ISO back to datetime-local format
  const displayValue =
    field.type === "datetime" && value ? isoToDatetimeLocal(value) : value;

  // Render enum dropdown
  if (field.enum) {
    return (
      <div className="space-y-1">
        <label className="text-xs font-medium text-charcoal-700 dark:text-steel-300">
          {field.label}
          {field.required && <span className="text-red-500 ml-1">*</span>}
        </label>
        <select
          value={value}
          onChange={(e) => onChange(e.target.value)}
          className="w-full p-2 text-sm border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-lg text-charcoal-900 dark:text-white"
          required={field.required}
        >
          <option value="">Select...</option>
          {field.enum.map((option) => (
            <option key={option} value={option}>
              {option
                .replace(/_/g, " ")
                .replace(/\b\w/g, (l) => l.toUpperCase())}
            </option>
          ))}
        </select>
        {field.helpText && (
          <p className="text-xs text-charcoal-500 dark:text-steel-500">
            {field.helpText}
          </p>
        )}
      </div>
    );
  }

  // Render text/number input
  return (
    <div className="space-y-1">
      <div className="flex items-center justify-between">
        <label className="text-xs font-medium text-charcoal-700 dark:text-steel-300">
          {field.label}
          {field.required && <span className="text-red-500 ml-1">*</span>}
        </label>
        {field.unit && (
          <span className="text-xs text-charcoal-400 dark:text-steel-600 font-mono">
            {field.unit}
          </span>
        )}
      </div>
      <input
        type={
          field.type === "datetime"
            ? "datetime-local"
            : field.type === "number" || field.type === "integer"
            ? "number"
            : "text"
        }
        value={displayValue}
        onChange={handleChange}
        placeholder={field.placeholder}
        step={field.type === "integer" ? "1" : field.step || "0.1"}
        className="w-full p-2 text-sm border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-lg text-charcoal-900 dark:text-white"
        required={field.required}
      />
      {field.helpText && (
        <p className="text-xs text-charcoal-500 dark:text-steel-500">
          {field.helpText}
        </p>
      )}
    </div>
  );
};

/**
 * Create an empty item based on schema
 */
function createEmptyItem(schema) {
  const item = {};

  schema.fields.forEach((field) => {
    if (field.default !== undefined) {
      item[field.name] = field.default;
    } else if (field.type === "number" || field.type === "integer") {
      item[field.name] = "";
    } else if (field.type === "boolean") {
      item[field.name] = false;
    } else {
      item[field.name] = "";
    }
  });

  return item;
}

ArrayInput.propTypes = {
  label: PropTypes.string.isRequired,
  name: PropTypes.string.isRequired,
  value: PropTypes.array,
  onChange: PropTypes.func.isRequired,
  itemSchema: PropTypes.shape({
    fields: PropTypes.arrayOf(
      PropTypes.shape({
        name: PropTypes.string.isRequired,
        label: PropTypes.string.isRequired,
        type: PropTypes.oneOf([
          "string",
          "number",
          "integer",
          "boolean",
          "datetime",
        ]).isRequired,
        unit: PropTypes.string,
        required: PropTypes.bool,
        enum: PropTypes.arrayOf(PropTypes.string),
        default: PropTypes.any,
        placeholder: PropTypes.string,
        helpText: PropTypes.string,
        step: PropTypes.string,
      })
    ).isRequired,
  }).isRequired,
  required: PropTypes.bool,
  helpText: PropTypes.string,
  t: PropTypes.object,
};

ArrayItemField.propTypes = {
  field: PropTypes.object.isRequired,
  value: PropTypes.any,
  onChange: PropTypes.func.isRequired,
};

export default ArrayInput;
