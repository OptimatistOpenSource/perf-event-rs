/*
struct {
  u64    sample_id;   /* if PERF_SAMPLE_IDENTIFIER */
  u64    ip;          /* if PERF_SAMPLE_IP */
  u32    pid, tid;    /* if PERF_SAMPLE_TID */
  u64    time;        /* if PERF_SAMPLE_TIME */
  u64    addr;        /* if PERF_SAMPLE_ADDR */
  u64    id;          /* if PERF_SAMPLE_ID */
  u64    stream_id;   /* if PERF_SAMPLE_STREAM_ID */
  u32    cpu, res;    /* if PERF_SAMPLE_CPU */
  u64    period;      /* if PERF_SAMPLE_PERIOD */
  struct read_format v;
                      /* if PERF_SAMPLE_READ */
  u64    nr;          /* if PERF_SAMPLE_CALLCHAIN */
  u64    ips[nr];     /* if PERF_SAMPLE_CALLCHAIN */
  u32    size;        /* if PERF_SAMPLE_RAW */
  char   data[size];  /* if PERF_SAMPLE_RAW */
  u64    bnr;         /* if PERF_SAMPLE_BRANCH_STACK */
  struct perf_branch_entry lbr[bnr];
                      /* if PERF_SAMPLE_BRANCH_STACK */
  u64    abi;         /* if PERF_SAMPLE_REGS_USER */
  u64    regs[weight(mask)];
                      /* if PERF_SAMPLE_REGS_USER */
  u64    size;        /* if PERF_SAMPLE_STACK_USER */
  char   data[size];  /* if PERF_SAMPLE_STACK_USER */
  u64    dyn_size;    /* if PERF_SAMPLE_STACK_USER &&
                         size != 0 */
  union perf_sample_weight weight;
                      /* if PERF_SAMPLE_WEIGHT */
                      /* || PERF_SAMPLE_WEIGHT_STRUCT */
  u64    data_src;    /* if PERF_SAMPLE_DATA_SRC */
  u64    transaction; /* if PERF_SAMPLE_TRANSACTION */
  u64    abi;         /* if PERF_SAMPLE_REGS_INTR */
  u64    regs[weight(mask)];
                      /* if PERF_SAMPLE_REGS_INTR */
  u64    phys_addr;   /* if PERF_SAMPLE_PHYS_ADDR */
  u64    cgroup;      /* if PERF_SAMPLE_CGROUP */
  u64    data_page_size;
                    /* if PERF_SAMPLE_DATA_PAGE_SIZE */
  u64    code_page_size;
                    /* if PERF_SAMPLE_CODE_PAGE_SIZE */
  u64    size;        /* if PERF_SAMPLE_AUX */
  char   data[size];  /* if PERF_SAMPLE_AUX */
};
*/

#[repr(C)]
pub struct Body {
    sample_id: u64,
    ip: u64,
    pid: u32,
    tid: u32,
    time: u64,
    addr: u64,
    id: u64,
    stream_id: u64,
    cpu: u32,
    res: u32,
    period: u64,
    // TODO
    nr: u64,
    // TODO
    size1: u32,
    // TODO
    bnr: u64,
    // TODO
    abi1: u64,
    // TODO
    size2: u64,
    // TODO
    dyn_size: u64,
    // TODO
    data_src: u64,
    transaction: u64,
    abi: u64,
    // TODO
    phys_addr: u64,
    cgroup: u64,
    data_page_size: u64,
    code_page_size: u64,
    size3: u64,
    // TODO
}
