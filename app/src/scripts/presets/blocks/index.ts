import { colors } from "../../utils/colors";
import { createRecapPresets } from "../templates/recap";

export function createPresets(scale: ResourceScale) {
  return {
    name: "Blocks",
    tree: [
      ...((scale === "date"
        ? [
            {
              scale,
              icon: IconTablerWall,
              name: "Height",
              title: "Block Height",
              description: "",
              unit: "Height",
              bottom: [
                {
                  title: "Height",
                  color: colors.bitcoin,
                  datasetPath: `/date-to-last-height`,
                },
              ],
            },
            {
              scale,
              name: "Mined",
              tree: [
                {
                  scale,
                  icon: IconTablerCube,
                  name: "Daily Sum",
                  title: "Daily Sum Of Blocks Mined",
                  description: "",
                  unit: "Count",
                  bottom: [
                    {
                      title: "Target",
                      color: colors.white,
                      datasetPath: `/date-to-blocks-mined-1d-target`,
                      options: {
                        lineStyle: 3,
                      },
                    },
                    {
                      title: "1W Avg.",
                      color: colors.momentumYellow,
                      datasetPath: `/date-to-blocks-mined-1w-sma`,
                      defaultVisible: false,
                    },
                    {
                      title: "1M Avg.",
                      color: colors.bitcoin,
                      datasetPath: `/date-to-blocks-mined-1m-sma`,
                    },
                    {
                      title: "Mined",
                      color: colors.darkBitcoin,
                      datasetPath: `/date-to-blocks-mined`,
                    },
                  ],
                },
                {
                  scale,
                  icon: IconTablerLetterW,
                  name: "Weekly Sum",
                  title: "Weekly Sum Of Blocks Mined",
                  description: "",
                  unit: "Count",
                  bottom: [
                    {
                      title: "Target",
                      color: colors.white,
                      datasetPath: `/date-to-blocks-mined-1w-target`,
                      options: {
                        lineStyle: 3,
                      },
                    },
                    {
                      title: "Sum Mined",
                      color: colors.bitcoin,
                      datasetPath: `/date-to-blocks-mined-1w-sum`,
                    },
                  ],
                },
                {
                  scale,
                  icon: IconTablerLetterM,
                  name: "Monthly Sum",
                  title: "Monthly Sum Of Blocks Mined",
                  description: "",
                  unit: "Count",
                  bottom: [
                    {
                      title: "Target",
                      color: colors.white,
                      datasetPath: `/date-to-blocks-mined-1m-target`,
                      options: {
                        lineStyle: 3,
                      },
                    },
                    {
                      title: "Sum Mined",
                      color: colors.bitcoin,
                      datasetPath: `/date-to-blocks-mined-1m-sum`,
                    },
                  ],
                },
                {
                  scale,
                  icon: IconTablerLetterY,
                  name: "Yearly Sum",
                  title: "Yearly Sum Of Blocks Mined",
                  description: "",
                  unit: "Count",
                  bottom: [
                    {
                      title: "Target",
                      color: colors.white,
                      datasetPath: `/date-to-blocks-mined-1y-target`,
                      options: {
                        lineStyle: 3,
                      },
                    },
                    {
                      title: "Sum Mined",
                      color: colors.bitcoin,
                      datasetPath: `/date-to-blocks-mined-1y-sum`,
                    },
                  ],
                },
                {
                  scale,
                  icon: IconTablerWall,
                  name: "Total",
                  title: "Total Blocks Mined",
                  description: "",
                  unit: "Count",
                  bottom: [
                    {
                      title: "Mined",
                      color: colors.bitcoin,
                      datasetPath: `/date-to-total-blocks-mined`,
                    },
                  ],
                },
              ],
            },
            {
              scale,
              name: "Size",
              tree: createRecapPresets({
                scale,
                title: "Block Size",
                color: colors.darkWhite,
                unit: "Megabytes",
                keySum: "/date-to-block-size-1d-sum",
                keyAverage: "/date-to-block-size-1d-average",
                keyMax: "/date-to-block-size-1d-max",
                key90p: "/date-to-block-size-1d-90p",
                key75p: "/date-to-block-size-1d-75p",
                keyMedian: "/date-to-block-size-1d-median",
                key25p: "/date-to-block-size-1d-25p",
                key10p: "/date-to-block-size-1d-10p",
                keyMin: "/date-to-block-size-1d-min",
              }),
            },
            {
              scale,
              name: "Weight",
              tree: createRecapPresets({
                scale,
                title: "Block Weight",
                color: colors.darkWhite,
                unit: "Weight",
                keyAverage: "/date-to-block-weight-1d-average",
                keyMax: "/date-to-block-weight-1d-max",
                key90p: "/date-to-block-weight-1d-90p",
                key75p: "/date-to-block-weight-1d-75p",
                keyMedian: "/date-to-block-weight-1d-median",
                key25p: "/date-to-block-weight-1d-25p",
                key10p: "/date-to-block-weight-1d-10p",
                keyMin: "/date-to-block-weight-1d-min",
              }),
            },
            {
              scale,
              name: "VBytes",
              tree: createRecapPresets({
                scale,
                title: "Block VBytes",
                color: colors.darkWhite,
                unit: "Virtual Bytes",
                keyAverage: "/date-to-block-vbytes-1d-average",
                keyMax: "/date-to-block-vbytes-1d-max",
                key90p: "/date-to-block-vbytes-1d-90p",
                key75p: "/date-to-block-vbytes-1d-75p",
                keyMedian: "/date-to-block-vbytes-1d-median",
                key25p: "/date-to-block-vbytes-1d-25p",
                key10p: "/date-to-block-vbytes-1d-10p",
                keyMin: "/date-to-block-vbytes-1d-min",
              }),
            },
            {
              scale,
              name: "Interval",
              tree: createRecapPresets({
                scale,
                title: "Block Interval",
                color: colors.darkWhite,
                unit: "Seconds",
                keyAverage: "/date-to-block-interval-1d-average",
                keyMax: "/date-to-block-interval-1d-max",
                key90p: "/date-to-block-interval-1d-90p",
                key75p: "/date-to-block-interval-1d-75p",
                keyMedian: "/date-to-block-interval-1d-median",
                key25p: "/date-to-block-interval-1d-25p",
                key10p: "/date-to-block-interval-1d-10p",
                keyMin: "/date-to-block-interval-1d-min",
              }),
            },
          ]
        : [
            {
              scale,
              icon: IconTablerMaximize,
              name: "Size",
              title: "Block Size",
              description: "",
              unit: "Megabytes",
              bottom: [
                {
                  title: "Size",
                  color: colors.darkWhite,
                  datasetPath: `/height-to-block-size`,
                },
              ],
            },
            {
              scale,
              icon: IconTablerWeight,
              name: "Weight",
              title: "Block Weight",
              description: "",
              unit: "Weight",
              bottom: [
                {
                  title: "Weight",
                  color: colors.darkWhite,
                  datasetPath: `/height-to-block-weight`,
                },
              ],
            },
            {
              scale,
              icon: IconTablerBinary,
              name: "VBytes",
              title: "Block VBytes",
              description: "",
              unit: "Virtual Bytes",
              bottom: [
                {
                  title: "VBytes",
                  color: colors.darkWhite,
                  datasetPath: `/height-to-block-vbytes`,
                },
              ],
            },
            {
              scale,
              icon: IconTablerAlarm,
              name: "Interval",
              title: "Block Interval",
              description: "",
              unit: "Seconds",
              bottom: [
                {
                  title: "Interval",
                  color: colors.darkWhite,
                  datasetPath: `/height-to-block-interval`,
                },
              ],
            },
          ]) satisfies PartialPresetTree),
      {
        scale,
        icon: IconTablerStack3,
        name: "Cumulative Size",
        title: "Cumulative Block Size",
        description: "",
        unit: "Megabytes",
        bottom: [
          {
            title: "Size",
            color: colors.darkWhite,
            datasetPath: `/${scale}-to-cumulative-block-size`,
          },
        ],
      },
    ],
  } satisfies PartialPresetFolder;
}
